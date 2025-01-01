# WCE

WCE stands for "World Coding Entertainment", taken from famous WWE (World Wrestling Entertainment). But you might ask why?? Well, unlike WWE, here's stakes are high and injuries are real! Compete with live programmers and be ready to dominate the ring with the whims and intellect.

## Technical Specifications

The project is scaffolded from scratch and meant to serve as a potential MVP (Minimal Viable Product).

### Frontend

The frontend is made in a simple react app using vite.

###  Backend

The backend follows a microservice architecture and is implemented using Rust programming language. The backend microservice is split into 3 services each served in its own docker container.

![Backend Architecture Diagram](./assets/backend-arch.png)

1. Player

    This service handles all player related tasks like player rating, player signup/login, etc

2. Game

    This service handles all game logic related activities like player matching, game logic, etc

3. Editor

    This service handles all editor environment related tasks like remote code execution, evaluating test cases, etc