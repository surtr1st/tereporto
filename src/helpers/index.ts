export const removeQuotes = (target: string) => target.replaceAll('"', '');
export const generateRandomHexColor = () =>
  `#${Math.floor(Math.random() * 16777215).toString(16)}`;
