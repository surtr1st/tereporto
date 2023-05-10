<script setup lang="ts">
import { defineAsyncComponent, ref } from 'vue';
const Button = defineAsyncComponent(() => import('./components/Button.vue'));
const ButtonGroup = defineAsyncComponent(
  () => import('./components/ButtonGroup.vue'),
);
const Checkbox = defineAsyncComponent(
  () => import('./components/Checkbox.vue'),
);
const Descriptive = defineAsyncComponent(
  () => import('./components/Descriptive.vue'),
);
const FileSelection = defineAsyncComponent(
  () => import('./components/FileSelection.vue'),
);
const Flex = defineAsyncComponent(() => import('./components/Flex.vue'));
const FunctionalPanel = defineAsyncComponent(
  () => import('./components/FunctionalPanel.vue'),
);
const Grid = defineAsyncComponent(() => import('./components/Grid.vue'));
const Idol = defineAsyncComponent(() => import('./components/Idol.vue'));
const GridItem = defineAsyncComponent(
  () => import('./components/GridItem.vue'),
);
const List = defineAsyncComponent(() => import('./components/List.vue'));
const ListItem = defineAsyncComponent(
  () => import('./components/ListItem.vue'),
);
const Modal = defineAsyncComponent(() => import('./components/Modal.vue'));
const ModalFooter = defineAsyncComponent(
  () => import('./components/ModalFooter.vue'),
);
const StoragePanel = defineAsyncComponent(
  () => import('./components/StoragePanel.vue'),
);
const TeleportPanel = defineAsyncComponent(
  () => import('./components/TeleportPanel.vue'),
);
const TitleHeader = defineAsyncComponent(
  () => import('./components/TitleHeader.vue'),
);
const TrashIcon = defineAsyncComponent(
  () => import('./components/TrashIcon.vue'),
);
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
                name="teleport-trash-btn-"
                color="danger"
              >
                <TrashIcon />
              </Button>
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
                :name="'storage-trash-btn-' + i"
                color="danger"
              >
                <TrashIcon />
              </Button>
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
          <FileSelection
            label="New Teleport"
            name="teleport"
          />
          <FileSelection
            label="New Storage"
            name="storage"
          />
          <ButtonGroup>
            <Button
              name="scan-btn"
              label="Scan Teleport"
              color="darker"
              larger
            />
            <Button
              name="choose-btn"
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
  <Modal
    :open="open"
    title="Test"
    @close="open = false"
  >
    <ModalFooter>
      <Button
        label="Test"
        color="darker"
        rounded
        larger
      />
    </ModalFooter>
  </Modal>
</template>
