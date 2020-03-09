---
layout: post
title: "Check Your Assumptions"
date: 2020-03-09
tags: tech
author: frosty
---
## Why I was late
### Poor Planning
So this post was supposed to come at an earlier date, but I figured that I would hit the goal of putting a development tool into a parent window a lot sooner. I thought everything was cherry, so I went off and did a bunch of research for my streaming application and got a bit carried away. I came back a week and a half later and found a number of issues with how I imagined I would go forward with implementation.

### Technical Issues
When I finally went to implement tying up the winit windows together, by providing a parent id to the alacritty window, I found that the winit event loops require that they be run on the main thread. This meant that I would have to combine the event loop for alacritty with the amethyst event loop or run each as a separate process and pipe the inputs back and forth. The first idea seemed to go against my "be lazy" approach to the project that followed a path of least maintenance. The second idea was much dirtier and gave me less control over the messaging between alacritty and the game. I chose ever-prevailing third option, go back to the drawing board and find tools that better suit the problem.

### Old friend, New clothes: SDL
My first Google search attempted to find an embeddable text editor or terminal within the rust ecosystem that had a UI implementation. Xi kept on turning up. It seemed like a solid backend for a text editor, but many of the frontends were either not being maintained or they weren't using something that really fit well with game engines in the rust ecosystem. I broadened my search for any editor backends rust or not in hopes of finding something that was perhaps buried. Looks like neovim is pretty popular among enthusiasts. And look at that <https://github.com/Kethku/neovide>... a regularly maintained, nice looking editor that's in rust and using a popular libary for windowing (SDL). If I recall, the Piston engine has a SDL window backend.

## *HACKERMAN*
### Working with SDL
It's been years since I'd done anything with SDL, but it still feels somewhat comfortable despite using the rust bindings. So the goal is to put a text editor into a game with a sub window. 

First, I had to make sure I didn't run into the same issue with event loops not playing nice with each other. A cursory Google search of SDL2 with regard to multiple windowing assured me that SDL was capable of managing multiple windows. I hacked neovide into a library and imported it into the Piston SDL window example, plugged in the pump, and spun up the program. Looks like it's able to handle events from two windows with the same event loop just fine.

Second, I needed to keep the neovide window bound to the parent window, in other words, make it behave like a child window, since in reality the two windows are more like big brother and little brother than parent and child. 

*clickity clackity*
```rust
fn snap(b_window: &sdl2::video::Window, s_window: &mut sdl2::video::Window) {
    let b_pos = b_window.position();
    let s_pos = s_window.position();
    let b_size = b_window.size();
    let s_size = s_window.size();
    let mut snap_pos = s_pos;
    if s_pos.0 < b_pos.0 {
        snap_pos.0 = b_pos.0;
    }
    if s_pos.1 < b_pos.1 {
        snap_pos.1 = b_pos.1;
    }
    if b_pos.0 + (b_size.0 as i32) < s_pos.0 + (s_size.0 as i32) {
        snap_pos.0 = b_pos.0 + (b_size.0 as i32) - (s_size.0 as i32);
    }
    if b_pos.1 + (b_size.1 as i32) < s_pos.1 + (s_size.1 as i32) {
        snap_pos.1 = b_pos.1 + (b_size.1 as i32) - (s_size.1 as i32);
    }
    s_window.set_position(
        sdl2::video::WindowPos::Positioned(snap_pos.0),
        sdl2::video::WindowPos::Positioned(snap_pos.1),
    );
}
```
This is still missing a size check, but when I go in to refactor everything, I'll make sure to add that check, but for PoC purposes, this is satisfactory. I did some minor cleanup and went to bed.

## Closing
### Lesson
If I were working on any strict deadline or if I had an accountabili-buddy of sorts, I would probably have focused more on this project and had a clean blog post for my reader~~s~~, but let's not make any excuses. I acted like an irresponsible college student, and blew all the time I had indulging myself in browsing Github. I like shiny new toys, always have. I need to be more mindful of the time and recognize the actual constraints of the problem. Assumptions are death.

### On the next episode...
I want to think a bit more about the story elements of the project, so I'll write something about that and maybe include a bonus refactoring post for make up in being late.

See you in space Cowboy...