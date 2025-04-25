load("@aspect_bazel_lib//lib:write_source_files.bzl", "write_source_files")

genrule(
    name = "generate_helloworld",
    srcs = [],
    outs = ["out/helloworld.txt"],
    cmd = "echo \"Hello World\" > \"$@\""
)

genrule(
    name = "uppercase_python",
    srcs = [
        ".tool-versions",
        "//:generate_helloworld"
    ],
    outs = ["out/hello_upper.txt"],
    cmd = "python -c \"with open('$(location //:generate_helloworld)','r') as f: print('\\n'.join(f.readlines()).upper())\" > \"$@\""
)

write_source_files(
    name = "write_to_out",
    files = {
        "out/hello": "//:generate_helloworld",
        "out/hello_upper": "//:uppercase_python",
    },
)
