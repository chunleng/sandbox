load("@aspect_bazel_lib//lib:write_source_files.bzl", "write_source_files")

def hello(name):
    native.genrule(
        name = "gen_output_{}".format(name),
        srcs = [],
        outs = ["out/hello_world"],
        cmd = "echo hello, {} > $@".format(name)
    )

    write_source_files(
        name = "write_to_out_{}".format(name),
        files = {
            "out/hello": "//:gen_output_{}".format(name),
        },
    )


def _sum_and_output(ctx):
    if ctx.outputs.output != None:
        output_file = ctx.outputs.output
    else:
        output_file = ctx.actions.declare_file(ctx.label.name)
    ctx.actions.write(output_file, str(ctx.attr.num1 + ctx.attr.num2))
    return DefaultInfo(files = depset([output_file]))

sum_and_output = rule(
    implementation = _sum_and_output,
    attrs = {
        "num1": attr.int(),
        "num2": attr.int(),
        "output": attr.output(),
    }
)
