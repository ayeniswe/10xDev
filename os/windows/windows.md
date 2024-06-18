# Window Ecosystem

## Windows API (WinAPI)

<blockquote style="background: black;">
The Windows API Core components (Kernel, User, GDI, etc)
Common functions and libraries (kernel32.dll, user32.dll, gdi32.dll)
</blockquote>

### Window Creation

<blockquote style="background: black;">
All windows must be apart of a class
which is created with <code>RegisterClass</code>.
A window can only be apart of a <b>single</b> class. The class is reponsible for
handling messages that are sent to window; hence why class must have a window
process function to point to
</blockquote>

#### 💡Note

Strings for most api functions are based on `UTF-16 (W-Wide)`. The other string
type used occasionally is `UTF-8 (A-Ansi)`.

- ##### Example

    ```rust
    CreateWindowExA(), CreateWindowExW()
    ```

### Device Context

<blockquote style="background: black;">
A device context can be made <code>unique</code> or  <code>shared</code> among
all children windows in
a class
</blockquote>

### Handles

<blockquote style="background: black;">
A handle instance can have <b>multiple instances</b>
or a <b>single instance</b> if using the main process in itself;
commonly seen for dll or exe modules
</blockquote>
