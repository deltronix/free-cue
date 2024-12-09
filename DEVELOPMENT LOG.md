## Key Decisions
Lets define some decisions for this project...
### Which GUI library to use?
Slint and Iced are libraries that I have explored thus far. 
 - Slint:
  Pretty slick looking and easy to set up, but have to learn their language.
  - Iced
  Native rust is nice and I think I like the Elm architecture for UI development, easy to reason about. A bit too immature for my taste. Lacking more complex components.

This exploration has led me to Relm4. Based on GTK4 which is very mature UI library but with an Elm architecture bolted on top. So I have decided on using this for the GUI development of the FreeCue application.

## Milestones
 - M1: WAV playback in a sorted cue list.
 - M2: 
