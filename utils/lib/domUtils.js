export const appendToBody = item => {
  const text = document.createElement('p');
  text.innerText = item;
  document.body.appendChild(text);
};
