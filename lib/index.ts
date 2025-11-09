import init, { resize_inside } from "image-resizer-niki-rust"

export { init }

export const readFileAsDataURL = (file: File): Promise<string> => {
  return new Promise((resolve, reject) => {
    const reader = new FileReader()
    reader.onload = (e) => {
      const dataUrl = e.target?.result as string
      resolve(dataUrl)
    }
    reader.onerror = reject
    reader.readAsDataURL(file)
  })
}

export const loadImageFromDataURL = (
  imageUrl: string
): Promise<HTMLImageElement> => {
  return new Promise((resolve, reject) => {
    const img = new Image()
    img.onload = () => resolve(img)
    img.onerror = reject
    img.src = imageUrl
  })
}

export const loadImageFromFile = async (
  file: File
): Promise<HTMLImageElement> => {
  const dataUrl = await readFileAsDataURL(file)
  return loadImageFromDataURL(dataUrl)
}

export const createCanvasFromImage = (
  img: HTMLImageElement
): HTMLCanvasElement => {
  const canvas = document.createElement("canvas")
  canvas.width = img.width
  canvas.height = img.height

  const ctx = canvas.getContext("2d")
  if (!ctx) {
    throw new Error("Failed to get 2D context from canvas")
  }

  ctx.drawImage(img, 0, 0)
  return canvas
}

export const cleanupCanvas = (canvas: HTMLCanvasElement): void => {
  if (canvas.parentNode) {
    canvas.parentNode.removeChild(canvas)
  }
}

export const resize = async (
  img: HTMLImageElement,
  width?: number,
  height?: number
): Promise<string> => {
  const inputCanvas = createCanvasFromImage(img)

  try {
    const outputCanvas = resize_inside(inputCanvas, width, height)

    const resizedDataUrl = outputCanvas.toDataURL()

    cleanupCanvas(inputCanvas)
    cleanupCanvas(outputCanvas)

    return resizedDataUrl
  } catch (error) {
    cleanupCanvas(inputCanvas)
    throw error
  }
}
