# PolicyEngineDemo
This is a simple demo for a logic-based policy engine design. Currently this demo shows a simple example of using rust to parse In-toto trust metadata and prolog logic language to enforce customized policy check on the In-toto metadata.

## RUN
This demo requires rust and swi-prolog installation. For simplicity, a docker container is recommanded.


```
docker run -it --rm --name demo -v $PWD:/home iqicheng/rust-logic-engine-demo bash
cd home/
cargo run DemoData/Intoto/
cargo run DemoData/Intoto_fake/
```