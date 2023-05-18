export const removeQuotes = (target: string) => {
  const removeSingle = target.replaceAll("'", '');
  return removeSingle.replaceAll('"', '');
};
export const generateRandomHexColor = () =>
  `#${Math.floor(Math.random() * 16777215).toString(16)}`;
