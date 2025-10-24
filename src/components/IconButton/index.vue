<template>
  <div
    class="menu-item cursor-pointer rd-3px size-32px flex items-center justify-center"
    :class="{
      'hover:bg-#f5f5f5': true
    }"
  >
    <slot/>
  </div>
</template>
