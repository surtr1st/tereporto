<script setup lang="ts">
import Button from './components/Button.vue';
import ButtonGroup from './components/ButtonGroup.vue';
import Descriptive from './components/Descriptive.vue';
import FunctionalPanel from './components/FunctionalPanel.vue';
import Idol from './components/Idol.vue';
import List from './components/List.vue';
import ListItem from './components/ListItem.vue';
import StoragePanel from './components/StoragePanel.vue';
</script>

<template>
  <StoragePanel>
    <List>
      <ListItem>
        <Idol />
        <Descriptive
          title="Storage folder X"
          description="usr/bede/123"
        />
        <Button
          rounded
          color="danger"
        />
      </ListItem>
    </List>
  </StoragePanel>
</template>

<style scoped></style>
