# PolicyEngineDemo
This is a simple demo for a logic-based policy engine design. Currently this demo shows two simple examples for enforcing policy checks on TUF and In-toto trust metadata.

## RUN
This demo requires rust and swi-prolog installation. For simplicity, a docker container is recommanded.


```
docker run -it --rm --name demo -v $PWD:/home iqicheng/rust-logic-engine-demo bash
cd home/
cargo run intoto DemoData/Intoto/
cargo run intoto DemoData/Intoto_fake/
cargo run tuf DemoData/TUF/
```

## Example Output
The output is in the `Example_Output` folder. The `Check.pl` file is the matching procedure, all other files are facts and predicates from trust metadata. 
## Design Ideas 
This is a simple first-draft design blueprint.

![plot](./Figs/Chainguard-Workspace.png)
