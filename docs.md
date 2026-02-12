# Documentation
## Architecture
The goal is to create an engine that can work well asynchronously. This is so the engine can be multithreaded properly and allow for optimized CPU performace as time goes on. This however, makes it had to create readable and understandable code.\\

To ensure order and comprehensiveness, the different functionalities are split in their own components. The guideline is that a component, never needs a reference to another. To allow this, the engine utilizes the observer pattern. If a component has a task for another, this is done through the "event bus". The different components can assign different functions to "observe" specific messages, and can emit messages through the event bus which are then handled by the corresponding components.\\

The downside to this is that a function can't "wait" for any information in return. As of now, I haven't thought of a fix, and the event bus is likely to see an extreme amount of messages back and forth until a solution is found.

## Main
Main is used to create and start the different runtimes.\\

First, a "Multiple Producer Single Consumer" channel, mpsc channel for short, is created. It's sender is sent to each component and the winit application, and the reciever is sent to the event bus.\\
Then the asynchronous runtime is created, this is where the components reside so they can create and assign their own threads as needed. The synchronous runtime is created after that, and this as of now, exists purely for the winit application. This is because the winit application is thread blocking by design, having it's own thread allows for other components to continue in the background while complying to winit's application flow.\\
