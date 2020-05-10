# Seabook
Experimental (Emulated) Client and Unit-test for Apache Kafka in Rust. Based specifically on 'Kafka-Rust' - https://github.com/spicavigo/kafka-rust

The general idea is to develop a client (and unit-test framework) that can work with Kafka and Arrow (including Cassandra).

Since Kafka already has a reactive and incredibly fault-tolerant framework, and unit-testing for Event Stream Processing services/applications are already available, the purpose of this repository is to experiment with ideas.

Currently my idea is to unit-test then crash-test the reactiveness and reliability of any Event Stream Processing application/service.

Questions:
- How does the integrity of an event stream safeguard and persist when physical connection is cut or shutdown?
- Apache Arrow's Bitmap.Rs is an interesting program, but how can we expand on its functionality? Especially since it only tracks null values, lobbies data into an array; the question is, where does its optimisation end and its expansive or extensive utility begin?
- From Arrow to Kafka and then into Cassandra: How can we intercept the raw events, enriched events as well as the analysis alogrithms employed by the system/framework - and how can we further our unit-test with them?

*I'm learning how to make a Functional Reactive Program in Rust with this project.
Hopefully this can be used for educational purposes.*
