# Most Common Design Patterns

## Creation

### Builder

<blockquote style="background: black;">
A useful pattern for creating software that is <b>optional</b> in many areas; leading
to a step by step process.
</blockquote>

#### Advanced Usage

- Use a `Director` class to abstract
  building routines. A ideal candidate when
  many build steps are used and repeated

#### Problems Solved

- Avoids the needs of multiple subclasses

## Structural

### Decorator

<blockquote style="background: black;">
A pattern to wrap objects with new behaviors. A great candidate when extra
behavior is needed at runtime
</blockquote>

#### Problems Solved

- Avoids the bloated code that is created from extending many different subclasses
  from an object
- Avoids the obstacle of inheritance being static

## Behavior

### Observer

<blockquote style="background: black;">
A pattern to notify all subscribers to events that take place
</blockquote>

#### Advanced Usage

- A PubSub upgraded pattern can be implemented by using observer logic but instead
  denoting subjects that subscribers can opt-in to have access to

#### Problems Solved

- Avoids spamming multiple objects that
  do not need the info
- Avoid overhead due to sending so
  much info to all objects (objects could grow leading to disaster)
- Avoid the problem of future objects added that
  could be dynamic in nature
