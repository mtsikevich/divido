#:sdk Aspire.AppHost.Sdk@13.4.6
#:package CommunityToolkit.Aspire.Hosting.Rust@*
#:package Aspire.Hosting.PostgreSQL@*
#:package CommunityToolkit.Aspire.Hosting.LavinMQ@*

var builder = DistributedApplication.CreateBuilder(args);
var lavinMq = builder.AddLavinMQ("message-queue")
    .WithDataVolume("lavin-data");

builder.AddRustApp("rust-app","../crates/api")
    .WithHttpEndpoint(port: 8080, env: "PORT")
    .WithReference(lavinMq)
    .WaitFor(lavinMq);

builder.Build().Run();
