export type Teleport = {
  name: string;
  directory?: string;
  to?: string;
  color?: string;
};

export type Storage = {
  name: string;
  directory: string;
  constraint?: string;
  color?: string;
};

export type Directory = string;

export type TeleportResponse = {
  index: string;
  name: string;
  directories: string[];
  to: string;
  color: string;
};

export type StorageResponse = {
  index: string;
  name: string;
  directory: string;
  constraint: string;
  color: string;
};

export type MappedField = {
  field: string;
  value: string | number;
};

export type UpdateArgs = {
  filename: string;
  target: MappedField;
};

export type TeleportUpdateArgs = UpdateArgs;
export type StorageUpdateArgs = UpdateArgs;

export type Settings = {
  auto_scan: boolean;
  preferred_lang: string;
  close_mode: string;
};
