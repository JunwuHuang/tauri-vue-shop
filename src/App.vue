<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api'
import { nanoid } from 'nanoid/non-secure'

interface Todo {
  id: string,
  label: string,
  done: boolean,
  is_delete: boolean,
}

const loading = ref(true)
const ipt = ref('')
let todos = ref<Todo[]>([])
const getTodos = async () => {
  try {
    const list = await invoke<Todo[]>('get_todos')
    todos.value = list
  } catch (error) {
    console.log(error)
  } finally {
    loading.value = false
  }
}
onMounted(getTodos)
const newTodo = async () => {
  loading.value = true

  try {
    const ret = await invoke<boolean>('new_todo', {
      todo: {
        id: nanoid(),
        label: ipt.value,
        done: false,
        is_delete: false,
      }
    })
    console.log(ret)
    if (ret) {
      ipt.value = ''
      getTodos()
    }
  } catch (error) {
    console.log(error)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <label>
    <h2>Input Todo</h2>
    <div>
      <input type="text" v-model="ipt" @keyup.enter="newTodo" :readonly="loading">
    </div>
  </label>
  <h2 v-if="loading">Loading...</h2>
  <ul v-else>
    <li v-for="todo in todos" :key="todo.id">
      {{ todo.label }}
    </li>
  </ul>
</template>

<style>
#app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}
</style>
