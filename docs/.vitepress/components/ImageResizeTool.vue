<script setup lang="ts">
import { ref, onMounted } from "vue"
import { init, resizeImageWithWasm, loadImageFromDataURL } from "@lib/index"

const selectedFile = ref<File | null>(null)
const imagePreview = ref<string | null>(null)
const resizedImage = ref<string | null>(null)
const targetWidth = ref(200)
const targetHeight = ref(200)
const isResizing = ref(false)
const error = ref<string | null>(null)
const wasmInitialized = ref(false)

onMounted(async () => {
  try {
    await init()
    wasmInitialized.value = true
  } catch (err: any) {
    error.value = "Failed to initialize WASM module: " + err.message
  }
})

const handleFileChange = async (event: Event) => {
  const target = event.target
  const file = target.files?.[0]
  if (file && file.type.startsWith("image/")) {
    selectedFile.value = file
    resizedImage.value = null
    error.value = null

    const reader = new FileReader()
    reader.onload = async (e) => {
      imagePreview.value = e.target?.result as string

      // Auto-resize after image is loaded
      if (!wasmInitialized.value) return

      isResizing.value = true
      error.value = null

      try {
        const img = await loadImageFromDataURL(imagePreview.value)
        const resizedDataUrl = await resizeImageWithWasm(
          img,
          targetWidth.value,
          targetHeight.value
        )
        resizedImage.value = resizedDataUrl
      } catch (err: any) {
        error.value = "Failed to resize image: " + err.message
      } finally {
        isResizing.value = false
      }
    }
    reader.readAsDataURL(file)
  }
}
</script>

<template>
  <div
    style="
      margin: 2rem auto;
      max-width: 800px;
      padding: 2rem;
      border: 1px solid #ddd;
      border-radius: 8px;
    "
  >
    <h2>Image Resize Tool</h2>

    <div
      v-if="error"
      style="
        padding: 1rem;
        background-color: #fee;
        border: 1px solid #fcc;
        border-radius: 4px;
        margin-bottom: 1rem;
        color: #c00;
      "
    >
      <strong>Error:</strong> {{ error }}
    </div>

    <div
      v-if="!wasmInitialized"
      style="
        padding: 1rem;
        background-color: #fef3cd;
        border: 1px solid #ffc107;
        border-radius: 4px;
        margin-bottom: 1rem;
      "
    >
      Initializing WASM module...
    </div>

    <div>
      <h3>1. Upload Image</h3>
      <input
        type="file"
        accept="image/*"
        @change="handleFileChange"
        style="margin: 1rem 0; padding: 0.5rem"
      />
    </div>

    <div v-if="selectedFile" style="margin-top: 2rem">
      <p><strong>Selected file:</strong> {{ selectedFile.name }}</p>
      <p>
        <strong>Size:</strong> {{ (selectedFile.size / 1024).toFixed(2) }} KB
      </p>
      <p><strong>Type:</strong> {{ selectedFile.type }}</p>

      <div v-if="imagePreview" style="margin-top: 1rem">
        <h4>Original Image:</h4>
        <img
          :src="imagePreview"
          alt="Preview"
          style="
            max-width: 100%;
            height: auto;
            border: 1px solid #ccc;
            border-radius: 4px;
          "
        />
      </div>

      <div v-if="isResizing" style="margin-top: 2rem">
        <p>Resizing image...</p>
      </div>

      <div v-if="resizedImage" style="margin-top: 2rem">
        <h3>2. Resized Image</h3>
        <img
          :src="resizedImage"
          alt="Resized"
          style="
            max-width: 100%;
            height: auto;
            border: 1px solid #ccc;
            border-radius: 4px;
          "
        />
        <div style="margin-top: 1rem">
          <a
            :href="resizedImage"
            :download="'resized-' + selectedFile.name"
            style="
              padding: 0.5rem 1rem;
              background-color: #35495e;
              color: white;
              text-decoration: none;
              border-radius: 4px;
              display: inline-block;
            "
          >
            Download Resized Image
          </a>
        </div>
      </div>
    </div>
  </div>
</template>
