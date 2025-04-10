use std::{error::Error, sync::Arc};

use rustls::{
    ClientConfig, RootCertStore,
    crypto::aws_lc_rs,
    pki_types::{CertificateDer, PrivateKeyDer, pem::PemObject},
};
use tokio_postgres::connect;
use tokio_postgres_rustls::MakeRustlsConnect;

fn load_certs() -> Vec<CertificateDer<'static>> {
    CertificateDer::pem_file_iter("./data/certs/client.crt")
        .expect("cannot open certificate file")
        .map(|result| result.unwrap())
        .collect()
}

fn load_private_key() -> PrivateKeyDer<'static> {
    PrivateKeyDer::from_pem_file("./data/certs/client.key").expect("cannot read private key file")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let connection_string = "postgresql://root:password@localhost:5432/?sslmode=require";
    let root_cert_store = {
        let mut r = RootCertStore::empty();
        r.add_parsable_certificates(
            CertificateDer::pem_file_iter("./data/certs/server.crt")
                .expect("cannot open CA file")
                .map(|result| result.unwrap()),
        );
        r
    };
    let mut rustls_config = ClientConfig::builder()
        .with_root_certificates(root_cert_store)
        .with_client_auth_cert(load_certs(), load_private_key())?;

    // Skip CA check as the certificate was not properly created
    rustls_config.dangerous().set_certificate_verifier(Arc::new(
        danger::NoCertificateVerification::new(aws_lc_rs::default_provider()),
    ));

    let (client, connection) =
        connect(connection_string, MakeRustlsConnect::new(rustls_config)).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            println!("{:?}", e);
        }
    });

    let row = client.query("SELECT 'connected!'", &[]).await?;
    println!("{}", row[0].get::<usize, &str>(0));
    Ok(())
}

// ref: https://github.com/rustls/rustls/blob/c5b9b10147d8c448f251cbb77192daf9770e309e/examples/src/bin/tlsclient-mio.rs#L331
mod danger {
    use rustls::DigitallySignedStruct;
    use rustls::client::danger::HandshakeSignatureValid;
    use rustls::crypto::{CryptoProvider, verify_tls12_signature, verify_tls13_signature};
    use rustls::pki_types::{CertificateDer, ServerName, UnixTime};

    #[derive(Debug)]
    pub struct NoCertificateVerification(CryptoProvider);

    impl NoCertificateVerification {
        pub fn new(provider: CryptoProvider) -> Self {
            Self(provider)
        }
    }

    impl rustls::client::danger::ServerCertVerifier for NoCertificateVerification {
        fn verify_server_cert(
            &self,
            _end_entity: &CertificateDer<'_>,
            _intermediates: &[CertificateDer<'_>],
            _server_name: &ServerName<'_>,
            _ocsp: &[u8],
            _now: UnixTime,
        ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
            Ok(rustls::client::danger::ServerCertVerified::assertion())
        }

        fn verify_tls12_signature(
            &self,
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            verify_tls12_signature(
                message,
                cert,
                dss,
                &self.0.signature_verification_algorithms,
            )
        }

        fn verify_tls13_signature(
            &self,
            message: &[u8],
            cert: &CertificateDer<'_>,
            dss: &DigitallySignedStruct,
        ) -> Result<HandshakeSignatureValid, rustls::Error> {
            verify_tls13_signature(
                message,
                cert,
                dss,
                &self.0.signature_verification_algorithms,
            )
        }

        fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
            self.0.signature_verification_algorithms.supported_schemes()
        }
    }
}
