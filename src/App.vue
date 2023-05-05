<script setup lang="ts">
import Button from './components/Button.vue';
import Descriptive from './components/Descriptive.vue';
import Flex from './components/Flex.vue';
import FunctionalPanel from './components/FunctionalPanel.vue';
import Grid from './components/Grid.vue';
import GridItem from './components/GridItem.vue';
import Idol from './components/Idol.vue';
import List from './components/List.vue';
import ListItem from './components/ListItem.vue';
import StoragePanel from './components/StoragePanel.vue';
import TeleportPanel from './components/TeleportPanel.vue';
import TitleHeader from './components/TitleHeader.vue';
</script>

<template>
  <Grid>
    <GridItem position="lheader">
      <TitleHeader title="Teleported Folders" />
    </GridItem>
    <GridItem position="rheader">
      <TitleHeader title="Storage Folders" />
    </GridItem>
    <GridItem position="main">
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
    </GridItem>
    <GridItem position="aside">
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
    </GridItem>
    <GridItem position="footer">
      <FunctionalPanel></FunctionalPanel>
    </GridItem>
  </Grid>
</template>

<style scoped></style>
