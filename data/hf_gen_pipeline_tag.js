// https://huggingface.co/front/build/kube-e130e7f/gguf-CS9oOPAE.js <-- find actual gguf script in sources
const u = {};

Object.values(u)
  .map((val) => {
    return `PipelineTag::${val.name
      .replaceAll(" ", "")
      .replaceAll("-", "")} => "${val.name}".to_string()`;
  })
  .join(",\n");
