import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import { Show, createEffect, createSignal, onMount } from "solid-js";
import {
  Format,
  checkPermissions,
  requestPermissions,
  scan,
} from "@tauri-apps/plugin-barcode-scanner";
import { message as messageDialog } from "@tauri-apps/plugin-dialog";

function DesktopApp() {
  let cropper: HTMLDivElement | undefined;
  let container: HTMLDivElement | undefined;

  const [x1, setX1] = createSignal(0);
  const [y1, setY1] = createSignal(0);
  const [x2, setX2] = createSignal(0);
  const [y2, setY2] = createSignal(0);

  function updateCropperPosition() {
    //This will restyle the div
    const x3 = Math.min(x1(), x2()); //Smaller X
    const x4 = Math.max(x1(), x2()); //Larger X
    const y3 = Math.min(y1(), y2()); //Smaller Y
    const y4 = Math.max(y1(), y2()); //Larger Y
    cropper!.style.left = x3 + "px";
    cropper!.style.top = y3 + "px";
    cropper!.style.width = x4 - x3 + "px";
    cropper!.style.height = y4 - y3 + "px";
  }

  onMount(() => {
    container!.onmousedown = (e) => {
      cropper!.style.display = "block";
      setX1(e.clientX);
      setY1(e.clientY);
      updateCropperPosition();
    };
    container!.onmousemove = (e) => {
      setX2(e.clientX);
      setY2(e.clientY);
      updateCropperPosition();
    };
    container!.onmouseup = async () => {
      const codes = await invoke<string[]>("scan_area", {
        x: x1(),
        y: y1(),
        width: x2() - x1(),
        height: y2() - y1(),
      });
      if (codes.length === 0) {
        cropper!.style.display = "none";
      }
    };
  });

  return (
    <div class="container" ref={container!}>
      <div
        ref={cropper!}
        style="display: none; position: absolute; border: 1px dotted #000;"
      ></div>
    </div>
  );
}

function MobileApp() {
  const [scanning, setScanning] = createSignal(false);
  const [message, setMessage] = createSignal("scan");

  async function doScan() {
    let permission = await checkPermissions();
    if (permission === "prompt") {
      permission = await requestPermissions();
    }
    if (permission === "granted") {
      const scanned = await scan({ formats: [Format.QRCode], windowed: true });
      await messageDialog(scanned.content, {
        kind: "info",
      });
      setMessage(scanned.content);
      setScanning(false);
    } else {
      const msg = "not allowed to use the camera";
      await messageDialog(msg, {
        kind: "error",
      });
      setMessage(msg);
      setScanning(false);
    }
  }

  createEffect(() => {
    if (scanning()) {
      doScan();
    }
  });

  return (
    <Show
      when={scanning()}
      fallback={
        <div>
          <button
            style="width: 100vw; height: 100vh;"
            onClick={() => setScanning(true)}
          >
            {message()}
          </button>
        </div>
      }
    >
      <div>
        <div class="sample-background"></div>
        <div class="container">
          <div class="barcode-scanner--area--container">
            <div class="relative">
              <p>Aim your camera at a QR Code</p>
            </div>
            <div class="square surround-cover">
              <div class="barcode-scanner--area--outer surround-cover">
                <div class="barcode-scanner--area--inner"></div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </Show>
  );
}

function App() {
  if (
    import.meta.env.TAURI_ENV_PLATFORM === "windows" ||
    import.meta.env.TAURI_ENV_PLATFORM === "linux" ||
    import.meta.env.TAURI_ENV_PLATFORM === "darwin"
  ) {
    return DesktopApp();
  } else {
    return MobileApp();
  }
}

export default App;
