# Rust Survival Guide

## Strings

<blockquote style="background: black;">
<code>to_string()</code> is allocating new memory on the heap so if you try to
get a pointer from a <code>str</code> that has been converted to a
<code>String</code> you will be getting a totally new address.
</blockquote>

- ### Example

  ```rust
  /// Would be false since new memory is being allocated
  string_ptr1 = "test".to_string().as_ptr()
  string_ptr2 = "test".to_string().as_ptr()
  assert(string_ptr1 == string_ptr2, false)

  /// Would be true since no new memory is allocated
  string_ptr1 = "test".as_ptr()
  string_ptr2 = "test".as_ptr()
  assert(string_ptr1 == string_ptr2, true)
  ```
