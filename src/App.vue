<script lang="ts">
import { defineAsyncComponent, onMounted, ref, watch } from 'vue';
import { StorageResponse, TeleportResponse } from './types';
import { useTeleport } from './server';
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
const DirectoryChooser = defineAsyncComponent(
  () => import('./components/DirectoryChooser.vue'),
);
const Flex = defineAsyncComponent(() => import('./components/Flex.vue'));
const FunctionalPanel = defineAsyncComponent(
  () => import('./components/FunctionalPanel.vue'),
);
const Grid = defineAsyncComponent(() => import('./components/Grid.vue'));
const GridItem = defineAsyncComponent(
  () => import('./components/GridItem.vue'),
);
const Idol = defineAsyncComponent(() => import('./components/Idol.vue'));
const FolderTransferIcon = defineAsyncComponent(
  () => import('./components/Icon/FolderTransferIcon.vue'),
);
const FolderDestinationIcon = defineAsyncComponent(
  () => import('./components/Icon/FolderDestinationIcon.vue'),
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
  () => import('./components/Icon/TrashIcon.vue'),
);
</script>

<script setup lang="ts">
const open = ref<boolean>(false);
const teleport = ref<string | string[]>('');
const storage = ref<string | string[]>('');
const teleports = ref<TeleportResponse[] | undefined>([]);
const storages = ref<StorageResponse[] | undefined>([]);
const { getTeleports, createTeleport } = useTeleport();

function createNewTeleport() {
  // Get the last element which is the folder name
  const name = `${teleport.value}`.split('/').at(-1) as string;
  let directories: string[] = [];

  if (Array.isArray(teleport.value)) directories = teleport.value;
  else directories.push(teleport.value);

  createTeleport({ name, directories })
    .then((rs) => console.log(rs))
    .catch((e) => console.log(e));
}

watch(teleport, (newTeleport, oldTeleport) => {
  if (newTeleport.length === 0) return;
  createNewTeleport();
});

// onMounted(() => {
//   getTeleports()
//     .then((res) => (teleports.value = res))
//     .catch((e) => console.log(e));
// });
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
            <ListItem
              v-for="(teleport, index) in teleports"
              :id="teleport.index"
              :key="teleport.index"
            >
              <Idol>
                <FolderTransferIcon />
              </Idol>
              <Descriptive
                :title="teleport.name"
                :description="teleport.directories"
              />
              <Button
                rounded
                :name="'teleport-trash-btn-' + index"
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
              v-for="(storage, index) in storages"
              :id="storage.index"
              :key="storage.index"
            >
              <Idol>
                <FolderDestinationIcon />
              </Idol>
              <Descriptive
                :title="storage.name"
                :description="storage.directory"
              />
              <Button
                rounded
                :name="'storage-trash-btn-' + index"
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
          <DirectoryChooser
            label="New Teleport"
            name="teleport"
            v-model:select="teleport"
          />
          <DirectoryChooser
            label="New Storage"
            name="storage"
            v-model:select="storage"
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
