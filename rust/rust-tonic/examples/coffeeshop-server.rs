use rust_tonic::coffee::{
    coffeeshop_server::{self, Coffeeshop},
    BuyCoffeeResponse, CheckCoffeeResponse, CoffeeOrder, OrderInformation, OrderStatus,
};
use std::{collections::HashMap, sync::LazyLock, time::Duration};
use tokio::{
    spawn,
    sync::{mpsc, Mutex},
    time::sleep,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{transport::Server, Request, Response, Status};

struct OrderDetails {
    status: OrderStatus,
}

static NEXT_ID: LazyLock<Mutex<u32>> = LazyLock::new(|| Mutex::new(0u32));
static ORDERS: LazyLock<Mutex<HashMap<String, OrderDetails>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Default)]
struct LocalCoffeeshop {}

#[tonic::async_trait]
impl Coffeeshop for LocalCoffeeshop {
    type CheckCoffeeStream = ReceiverStream<Result<CheckCoffeeResponse, Status>>;

    async fn buy_coffee(
        &self,
        _: Request<CoffeeOrder>,
    ) -> Result<Response<BuyCoffeeResponse>, Status> {
        let id = {
            let mut next_id = NEXT_ID.lock().await;
            let id = next_id.clone();
            *next_id += 1;
            id.to_string()
        };

        {
            let mut orders = ORDERS.lock().await;
            orders.insert(
                id.clone(),
                OrderDetails {
                    status: OrderStatus::Queued,
                },
            );
        }

        spawn({
            let id = id.clone();
            async move {
                sleep(Duration::new(3, 0)).await;
                {
                    let mut orders = ORDERS.lock().await;
                    let order = orders.get_mut(&id).unwrap();
                    order.status = OrderStatus::Preparing;
                }
                sleep(Duration::new(10, 0)).await;
                {
                    let mut orders = ORDERS.lock().await;
                    let order = orders.get_mut(&id).unwrap();
                    order.status = OrderStatus::Ready;
                }
            }
        });

        Ok(Response::new(BuyCoffeeResponse {
            order_id: Some(id.to_string()),
        }))
    }

    async fn check_coffee(
        &self,
        request: Request<OrderInformation>,
    ) -> Result<Response<Self::CheckCoffeeStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        spawn({
            async move {
                let mut last_status = None::<OrderStatus>;
                loop {
                    let status = {
                        let orders = ORDERS.lock().await;
                        let order = orders.get(&request.get_ref().order_id);
                        if order.is_none() {
                            tx.send(Err(Status::invalid_argument("Invalid order_id")))
                                .await
                                .unwrap();
                            break;
                        }
                        order.unwrap().status
                    };

                    if last_status != Some(status) {
                        tx.send(Ok(CheckCoffeeResponse {
                            order_status: status.into(),
                        }))
                        .await
                        .unwrap();
                        last_status = Some(status);
                    }
                    if status == OrderStatus::Ready {
                        break;
                    }
                    sleep(Duration::new(1, 0)).await;
                }
            }
        });
        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() {
    Server::builder()
        .add_service(coffeeshop_server::CoffeeshopServer::new(
            LocalCoffeeshop::default(),
        ))
        .serve("127.0.0.1:3000".parse().unwrap())
        .await
        .unwrap();
}
