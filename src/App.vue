<script setup lang="ts">
import Button from './components/Button.vue';
import ButtonGroup from './components/ButtonGroup.vue';
import Checkbox from './components/Checkbox.vue';
import Descriptive from './components/Descriptive.vue';
import Flex from './components/Flex.vue';
import FunctionalPanel from './components/FunctionalPanel.vue';
import Grid from './components/Grid.vue';
import GridItem from './components/GridItem.vue';
import Idol from './components/Idol.vue';
import List from './components/List.vue';
import ListItem from './components/ListItem.vue';
import Modal from './components/Modal.vue';
import StoragePanel from './components/StoragePanel.vue';
import TeleportPanel from './components/TeleportPanel.vue';
import TitleHeader from './components/TitleHeader.vue';
import { ref } from 'vue';
const open = ref<boolean>(false);
</script>

<template>
  <Grid>
    <GridItem position="lheader">
      <Flex
        justify-content="center"
        align-items="center"
      >
        <TitleHeader
          title="Teleported Folders"
          text-position="center"
        />
      </Flex>
    </GridItem>
    <GridItem position="rheader">
      <Flex
        justify-content="center"
        align-items="center"
      >
        <TitleHeader
          title="Storage Folders"
          text-position="center"
        />
      </Flex>
    </GridItem>
    <GridItem position="main">
      <Flex justify-content="flex-end">
        <TeleportPanel>
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
        </TeleportPanel>
      </Flex>
    </GridItem>
    <GridItem position="aside">
      <Flex justify-content="flex-start">
        <StoragePanel>
          <List>
            <ListItem
              v-for="i in 15"
              :id="i"
            >
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
      </Flex>
    </GridItem>
    <GridItem position="footer">
      <Flex justify-content="center">
        <FunctionalPanel>
          <Checkbox label="Start along side with OS" />
          <Checkbox label="Auto-scan" />
          <Button
            label="New Teleport"
            color="darker"
            rounded
            larger
          />
          <Button
            label="New Storage"
            color="darker"
            rounded
            larger
          />
          <ButtonGroup>
            <Button
              label="Scan Teleport"
              color="darker"
              larger
            />
            <Button
              label="Choose"
              color="darker"
              larger
              @click="open = true"
            />
          </ButtonGroup>
        </FunctionalPanel>
      </Flex>
    </GridItem>
  </Grid>
  <Modal :open="open" />
</template>
