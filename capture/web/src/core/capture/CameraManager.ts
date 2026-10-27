export class CameraManager {
  private static stream: MediaStream | null = null;

  static async getAvailableCameras(): Promise<MediaDeviceInfo[]> {
    try {
      const tempStream = await navigator.mediaDevices.getUserMedia({
        video: true,
      });
      tempStream.getTracks().forEach((track) => track.stop());
    } catch (e) {
      console.warn(
        "Нет начального доступа к камере для чтения списка устройств: ",
        e,
      );
    }

    const devices = await navigator.mediaDevices.enumerateDevices();
    return devices.filter((device) => device.kind === "videoinput");
  }

  static async start(
    video: HTMLVideoElement,
    deviceId?: string,
  ): Promise<void> {
    this.stop(video);

    try {
      const constraints: MediaStreamConstraints = {
        video: deviceId
          ? { deviceId: { exact: deviceId } }
          : { facingMode: "user" },
      };

      this.stream = await navigator.mediaDevices.getUserMedia(constraints);
      video.srcObject = this.stream;

      return new Promise((resolve) => {
        video.onloadedmetadata = () => {
          video.play();
          resolve();
        };
      });
    } catch (error) {
      console.error("Ошибка инициализации камеры: ", error);
      throw error;
    }
  }

  static stop(video: HTMLVideoElement): void {
    if (this.stream) {
      this.stream.getTracks().forEach((track) => track.stop());
      this.stream = null;
    }
    if (video.srcObject) {
      video.srcObject = null;
    }
  }
}
