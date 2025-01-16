import languages from "./lang2hex.json";

let startY = 20;

const html = Object.entries(languages)
  .map(([lang, color]) => {
    const data = `<g>
    <rect x="20" y="${startY}" width="12" height="12" rx="6" fill="${color}" />
    <text x="38" y="${
      startY + 11
    }" fill="#CAD3F5" class="stat-text">${lang} 99.99%</text>
  </g>`;

    startY += 24;

    return data;
  })
  .join("\n");

console.log(`<svg
  width="512"
  height="4096"
  viewBox="0 0 512 4096"
  fill="none"
  xmlns="http://www.w3.org/2000/svg"
>
<rect width="512" height="4096" rx="6" fill="#24273A" />
  <g>${html}</g>
</svg>
`);
