<script lang="ts">
  // Static help content; no state.
</script>

<section class="help" aria-labelledby="help-title">
  <h2 id="help-title">Can’t connect?</h2>

  <details>
    <summary>Linux</summary>
    <p>
      Chrome needs permission to open the controller, and the built-in <code
        >xpad</code
      > driver has to let go of it. Allow access once:
    </p>
    <pre><code
        >echo 'SUBSYSTEM=="usb", ATTRS&#123;idVendor&#125;=="045e", TAG+="uaccess"' \
  | sudo tee /etc/udev/rules.d/60-xbox-webusb.rules
sudo udevadm control --reload-rules && sudo udevadm trigger</code
      ></pre>
    <p>Then, with the controller plugged in, release it from the driver:</p>
    <pre><code
        >ls /sys/bus/usb/drivers/xpad/          # shows something like 1-4:1.0
echo -n 1-4:1.0 | sudo tee /sys/bus/usb/drivers/xpad/unbind</code
      ></pre>
    <p>
      Connect here and set the light. Then hand the controller back to
      <code>xpad</code> without unplugging it, so the setting stays:
    </p>
    <pre><code>echo -n 1-4:1.0 | sudo tee /sys/bus/usb/drivers/xpad/bind</code
      ></pre>
    <p>Unplugging the cable resets the light to its normal glow.</p>
  </details>

  <details>
    <summary>Windows</summary>
    <p>
      Windows’ own Xbox driver holds the controller, so the browser can’t open
      it. The only way around that is swapping the driver to WinUSB with
      <a href="https://zadig.akeo.ie/" rel="noreferrer" target="_blank">Zadig</a
      >. While WinUSB is installed the controller doesn’t work in games, and
      rolling the driver back in Device Manager restarts the controller, which
      resets the light. In practice this page is most useful on Linux and macOS.
    </p>
  </details>

  <details>
    <summary>macOS</summary>
    <p>
      Usually works straight away. If Chrome reports the controller is in use,
      quit Steam and any game that has it open, replug the cable and try again.
    </p>
  </details>

  <details>
    <summary>Bluetooth</summary>
    <p>
      Pair the controller in your system settings first, then use <em
        >Connect over Bluetooth</em
      >. Over Bluetooth the controller only accepts rumble commands, so the
      light can only be changed over a cable. Web Bluetooth can’t reach it at
      all: browsers block access to Bluetooth keyboards and gamepads.
    </p>
  </details>

  <p class="note">
    The controller doesn’t store the setting. It returns to its normal glow when
    it turns off or reconnects.
  </p>
</section>

<style>
  .help {
    display: grid;
    gap: 0.5rem;
  }

  details {
    border-top: 1px solid var(--line);
    padding: 0.25rem 0;
  }

  details:last-of-type {
    border-bottom: 1px solid var(--line);
  }

  summary {
    cursor: pointer;
    font-weight: 600;
  }

  details p {
    max-width: 68ch;
  }

  pre {
    overflow-x: auto;
    padding: 0.875rem 1rem;
    background: var(--well);
    border-radius: 0.5rem;
    font-size: 0.85rem;
    line-height: 1.5;
  }

  .note {
    color: var(--ink-soft);
    margin-top: 0.75rem;
  }
</style>
