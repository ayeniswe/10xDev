# Configuration

## Common

<blockquote style="background: black;">
Some legacy systems can display old SHA1 standard fingerprints. Setting
the <code>~/.ssh/config</code> to the below is ideal for only showing the most secure
at the time.
</blockquote>

``` sh
Host *
    FingerprintHash sha256
```
