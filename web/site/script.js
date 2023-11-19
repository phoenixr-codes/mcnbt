import init, { explore } from './pkg/mcnbt_web.js';

function dom(tag) {
  const elemRoot = document.createElement("blockquote");
  const elemIcon = document.createElement("img");
  const elemName = document.createElement("span");

  const type = tag.type;
  const name = tag.name;
  const payload = tag.payload;

  elemIcon.classList.add("nbt-icon");
  elemIcon.setAttribute("src", `/assets/icons/${type}.png`);
  elemIcon.setAttribute("alt", type);

  let elemPayload;
  switch (type) {
    case "byte":
    case "short":
    case "int":
    case "long":
    case "float":
    case "double":
      elemPayload = document.createElement("span");
      elemPayload.classList.add("nbt-value");
      elemPayload.classList.add("nbt-type-number");
      elemPayload.append(payload);
      elemRoot.append(elemPayload);
      break;

    case "string":
      elemPayload = document.createElement("span");
      elemPayload.classList.add("nbt-value");
      elemPayload.classList.add("nbt-type-string");
      elemPayload.append(payload);
      elemRoot.append(elemPayload);
      break;

    case "compound":
    case "list":
      for (const i of payload) {
        elemRoot.append(dom(i));
      }
      break;

    case "byte_array":
    case "int_array":
    case "long_array":
      for (const i of payload) {
        let dummy = { "type": type.replace(/_array$/, ""), "name": null, "payload": i };
        elemRoot.append(dom(dummy));
      }
      break;

    default:
      console.error(`Unknown type ${type}`);
  }

  if (name !== null) {
    elemName.classList.add("nbt-name");
    elemName.append(name);
    elemRoot.prepend(elemName);
  }
  elemRoot.prepend(elemIcon);
  return elemRoot;
}

const form = document.getElementById("form");

async function run() {
  await init();

  form.addEventListener("submit", function (event) {
    event.preventDefault();
    
    const file = document.getElementById("nbt-file").files[0];
    const bigEndian = document.getElementById("big-endian").checked;
    file
      .arrayBuffer()
      .then(function (buffer) {
        const view = new Uint8Array(buffer);
        try {
          const result = explore(view, bigEndian);
          //console.debug(JSON.stringify(result));
          form.after(dom(result));
        } catch (error) {
          window.alert(error);
        }
      })
      .catch(console.error);
  });
}

run();

