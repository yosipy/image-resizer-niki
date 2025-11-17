import { describe, it, expect } from "vitest"
import {
  readFileAsDataURL,
  loadImageFromDataURL,
  loadImageFromFile,
  createCanvasFromImage,
  cleanupCanvas,
} from "./index"

describe("image-resizer-niki", () => {
  describe("readFileAsDataURL", () => {
    it("should be a function", () => {
      expect(typeof readFileAsDataURL).toBe("function")
    })
  })

  describe("loadImageFromDataURL", () => {
    it("should be a function", () => {
      expect(typeof loadImageFromDataURL).toBe("function")
    })
  })

  describe("loadImageFromFile", () => {
    it("should be a function", () => {
      expect(typeof loadImageFromFile).toBe("function")
    })
  })

  describe("createCanvasFromImage", () => {
    it("should be a function", () => {
      expect(typeof createCanvasFromImage).toBe("function")
    })
  })

  describe("cleanupCanvas", () => {
    it("should be a function", () => {
      expect(typeof cleanupCanvas).toBe("function")
    })
  })
})
