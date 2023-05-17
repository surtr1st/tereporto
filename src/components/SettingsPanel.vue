<script setup lang="ts">
import Flex from './Flex.vue';
import Button from './Button.vue';
import RadioButton from './RadioButton.vue';
import Modal from './Modal.vue';
import ModalContent from './ModalContent.vue';
import ModalFooter from './ModalFooter.vue';
import Checkbox from './Checkbox.vue';
import PlaceHolder from './PlaceHolder.vue';
import { onMounted, ref, watch } from 'vue';
import { refresh } from '../globals';
import { useSettings } from '../server';
import { Settings } from '../types';
import { removeQuotes } from '../helpers';
import { useI18n } from 'vue-i18n';

interface IConnectionPanel {
  open?: boolean;
  title?: string;
  onClose?: () => void | Promise<void>;
}
const { onClose } = defineProps<IConnectionPanel>();
const { load, save } = useSettings();
const { t, locale } = useI18n();

const langs = [
  { name: 'VI', value: 'vi' },
  { name: 'EN', value: 'en' },
  { name: 'JP', value: 'ja' },
];

const systems = [
  { name: t('message.panel.settings.system.exit'), value: 'default' },
  { name: t('message.panel.settings.system.minimize'), value: 'minimized' },
];

const settings = ref<Settings>({
  auto_scan: false,
  preferred_lang: '',
  close_mode: '',
});

function loadSettings() {
  load()
    .then((res) => {
      if (res) {
        settings.value = res;
        locale.value = removeQuotes(res.preferred_lang);
      }
    })
    .catch((e) => console.log(e));
}

function setOptions() {
  const option = new Map<string, string>();
  Object.entries(settings.value).forEach(([key, value]) => {
    if (typeof value === 'string') option.set(key, `${removeQuotes(value)}`);
    else option.set(key, `${value}`);
  });
  save(option)
    .then(() => {
      onClose!();
      refresh.fetch != refresh.fetch;
    })
    .catch((e) => console.log(e));
}

watch(
  () => refresh.fetch,
  () => {
    loadSettings();
  },
);

onMounted(() => loadSettings());
</script>

<template>
  <Modal
    :open="open"
    :title="title"
    @close="onClose"
  >
    <ModalContent>
      <Flex
        justify-content="center"
        align-items="center"
        :margin-bottom="25"
        :margin-top="25"
      >
        <Flex
          align-items="flex-start"
          column
          :width="320"
        >
          <PlaceHolder :title="$t('message.panel.settings.auto.title')">
            <Checkbox
              id="auto-scan-checkbox"
              :label="$t('message.panel.settings.auto.scan')"
              :value="!settings.auto_scan ? true : false"
              v-model:checked="settings.auto_scan"
            />
          </PlaceHolder>
          <PlaceHolder :title="$t('message.panel.settings.lang.title')">
            <Flex
              justify-content="space-between"
              align-items="flex-start"
              :width="320"
              :gap="27"
            >
              <RadioButton
                v-for="(lang, index) in langs"
                :key="index"
                :id="'teleport-radio-' + index"
                name="teleport-radio"
                :label="lang.name"
                :value="lang.value"
                :checked="lang.value === removeQuotes(settings.preferred_lang)"
                v-model:selected="settings.preferred_lang"
              />
            </Flex>
          </PlaceHolder>
          <PlaceHolder :title="$t('message.panel.settings.system.title')">
            <Flex
              justify-content="space-between"
              align-items="flex-start"
              :width="320"
              :gap="27"
            >
              <RadioButton
                v-for="(option, index) in systems"
                :key="index"
                :id="'system-radio-' + index"
                name="system-radio"
                :label="option.name"
                :value="option.value"
                :checked="option.value === removeQuotes(settings.close_mode)"
                v-model:selected="settings.close_mode"
              />
            </Flex>
          </PlaceHolder>
        </Flex>
      </Flex>
    </ModalContent>
    <ModalFooter>
      <Flex
        align-items="center"
        justify-content="center"
      >
        <Button
          :label="$t('message.feature.save')"
          name="save-btn"
          color="neutral"
          larger
          rounded
          @click="setOptions()"
        />
      </Flex>
    </ModalFooter>
  </Modal>
</template>
