java_library(
    name = "greeter",
    srcs = ["src/Greeter.java"],
    visibility = ["//visibility:public"],
)

java_binary(
    name = "app",
    srcs = ["src/App.java"],
    main_class = "app.App",
    deps = [":greeter"],
)

