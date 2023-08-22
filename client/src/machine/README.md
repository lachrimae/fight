The `machine` module uses the idea of a hierarchical state machine (as presented in [Bob Nystrom's book](https://gameprogrammingpatterns.com/state.html)) to manage transitions between character actions. E.g., when crouching, an "attack" input should trigger a down-tilt attack, but when jumping it should trigger a neutral-air attack.

The module uses dynamic dispatch to traits implemented in the `characteristics` module at runtime, so there is a fair amount of indirection. See `client/src/characteristics/README.md` for more info.
