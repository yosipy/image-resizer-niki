---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "image-resize-niki"
  text: "Resize image on web browser."
  tagline: My great project tagline
  actions:
    - theme: brand
      text: Markdown Examples
      link: /markdown-examples
    - theme: alt
      text: API Examples
      link: /api-examples

features:
  - title: Feature A
    details: Lorem ipsum dolor sit amet, consectetur adipiscing elit
  - title: Feature B
    details: Lorem ipsum dolor sit amet, consectetur adipiscing elit
  - title: Feature C
    details: Lorem ipsum dolor sit amet, consectetur adipiscing elit
---

<script setup>
import { ref } from 'vue'

const selectedFile = ref(null)
const imagePreview = ref(null)

const handleFileChange = (event) => {
  const file = event.target.files[0]
  if (file && file.type.startsWith('image/')) {
    selectedFile.value = file

    const reader = new FileReader()
    reader.onload = (e) => {
      imagePreview.value = e.target.result
    }
    reader.readAsDataURL(file)
  }
}
</script>

<div style="margin: 2rem auto; max-width: 600px; padding: 2rem; border: 1px solid #ddd; border-radius: 8px;">
  <h2>Upload Image</h2>
  <input
    type="file"
    accept="image/*"
    @change="handleFileChange"
    style="margin: 1rem 0; padding: 0.5rem;"
  />
  <div v-if="selectedFile" style="margin-top: 1rem;">
    <p><strong>Selected file:</strong> {{ selectedFile.name }}</p>
    <p><strong>Size:</strong> {{ (selectedFile.size / 1024).toFixed(2) }} KB</p>
    <p><strong>Type:</strong> {{ selectedFile.type }}</p>
    <div v-if="imagePreview" style="margin-top: 1rem;">
      <img :src="imagePreview" alt="Preview" style="max-width: 100%; height: auto; border: 1px solid #ccc; border-radius: 4px;" />
    </div>
  </div>
</div>
