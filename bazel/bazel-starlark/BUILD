load("//:starlark.bzl", "sum_and_output", "hello")
load("@aspect_bazel_lib//lib:write_source_files.bzl", "write_source_files")

# This basically generate a function that generates rules
# Output of the function is 2 rules: write_to_out_john and gen_output_john
hello("john")

# sum_and_output is a rule that uses a function underneath to perform some action
sum_and_output(
    name = "gen_sum",
    num1 = 1,
    num2 = 5,
    output = None,
)

sum_and_output(
    name = "gen_sum_with_output",
    num1 = 1,
    num2 = 5,
    output = "some_temp_file",
)

write_source_files(
    name = "write_to_out",
    files = {
        "out/sum": "//:gen_sum",
        "out/sum_answer": "//:gen_sum_with_output",
    },
)
