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
      The controller stops working as a gamepad until you unplug and replug it,
      which also hands it back to <code>xpad</code>.
    </p>
  </details>

  <details>
    <summary>Windows</summary>
    <p>
      Windows’ own Xbox driver holds the controller, so the browser can’t open
      it. Microsoft’s free
      <a
        href="https://apps.microsoft.com/detail/9nblggh30xj3"
        rel="noreferrer"
        target="_blank">Xbox Accessories</a
      >
      app changes the same light setting without any driver changes.
    </p>
    <p>
      To use this page instead, swap the controller’s driver to WinUSB with
      <a href="https://zadig.akeo.ie/" rel="noreferrer" target="_blank">Zadig</a
      >. While WinUSB is installed the controller doesn’t work in games; roll
      back the driver in Device Manager to undo it.
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
    padding: 0.75rem 0;
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
