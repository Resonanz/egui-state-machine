# egui State Machine

A rudimentary state machine demo using enums for States and Transitions.

* ```main.rs``` sets up egui text and buttons and displays the various screens depending upon the State the state machine is in. Pressing a button selects a transition (e.g. model.transition such as AtoB) which is acted upon in ```model.rs```.  
* ```model.rs``` defines the enum states and transitions, and matches upon model.transition to update the State according to the transitions.
* ```fonts.rs``` loads fonts and set the aliases that are used inside RichText etc. to select the font.

As shown in the image below, the current state is "STATE A" and the available transitions are AtoA and AtoB.

![image](https://github.com/user-attachments/assets/55e88948-6783-4006-a6ab-d66be8fb9ebd)

<img width="529" alt="image" src="https://github.com/user-attachments/assets/e2231439-a1d6-40bf-8543-b96e03ca6bca">
