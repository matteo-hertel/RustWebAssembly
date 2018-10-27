export const appendToBody = item => {
  const text = document.createElement('p');
  text.innerText = item;
  document.body.appendChild(text);
};

export const appendImage = src => {
  const img = document.createElement('img');
  img.src = src;
  document.body.appendChild(img);
};
