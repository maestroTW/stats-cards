const DEFAULT_USERNAME = "ilyhalight";

const themes = [
  {
    value: "catpuccin-macchiato",
    label: "Catppuccin Macchiato",
  },
  {
    value: "dark",
    label: "Dark",
    disabled: true,
  },
  {
    value: "white",
    label: "White",
    disabled: true,
  },
];

const layouts = [
  // {
  //   value: "normal",
  //   label: "Normal",
  // },
  {
    value: "compact",
    label: "Compact",
  },
];

const period = [
  {
    value: "3_months",
    label: "3 Months",
  },
  {
    value: "6_months",
    label: "6 Months",
  },
  {
    value: "year",
    label: "Year",
  },
];

function createPopup(el) {
  const popupEl = document.createElement("div");
  popupEl.classList.add("popup");
  popupEl.appendChild(el);
  return popupEl;
}

function createDropdown({
  title,
  selected,
  options,
  search = false,
  arrow = true,
  onSelect = (_option) => {},
}) {
  let opened = false;

  const containerEl = document.createElement("div");
  containerEl.classList.add("dropdown");

  const globalHandler = (e) => {
    if (containerEl.contains(e.target) && containerEl !== e.target) {
      return;
    }

    close();
  };

  const open = () => {
    opened = true;
    containerEl.dataset.opened = true;
    window.addEventListener("click", globalHandler);
  };

  const close = () => {
    opened = false;
    containerEl.dataset.opened = false;
    window.removeEventListener("click", globalHandler);
  };

  const titleEl = document.createElement("span");
  titleEl.classList.add("dropdown-title");
  titleEl.textContent = title;

  const buttonEl = document.createElement("button");
  buttonEl.classList.add("button", "button_outline", "dropdown-button");
  const buttonContentEl = document.createElement("span");
  buttonContentEl.textContent = selected.label;
  buttonEl.appendChild(buttonContentEl);

  let arrowEl;
  if (arrow) {
    arrowEl = document.createElement("span");
    arrowEl.classList.add("dropdown-button__icon");
    arrowEl.innerHTML = `<svg
  xmlns="http://www.w3.org/2000/svg"
  width="14"
  height="14"
  viewBox="0 0 24 24"
>
  <g fill="none" fill-rule="evenodd">
    <path
      d="M24 0v24H0V0zM12.593 23.258l-.011.002l-.071.035l-.02.004l-.014-.004l-.071-.035q-.016-.005-.024.005l-.004.01l-.017.428l.005.02l.01.013l.104.074l.015.004l.012-.004l.104-.074l.012-.016l.004-.017l-.017-.427q-.004-.016-.017-.018m.265-.113l-.013.002l-.185.093l-.01.01l-.003.011l.018.43l.005.012l.008.007l.201.093q.019.005.029-.008l.004-.014l-.034-.614q-.005-.019-.02-.022m-.715.002a.02.02 0 0 0-.027.006l-.006.014l-.034.614q.001.018.017.024l.015-.002l.201-.093l.01-.008l.004-.011l.017-.43l-.003-.012l-.01-.01z"
    />
    <path
      fill="currentColor"
      d="M13.06 16.06a1.5 1.5 0 0 1-2.12 0l-5.658-5.656a1.5 1.5 0 1 1 2.122-2.121L12 12.879l4.596-4.596a1.5 1.5 0 0 1 2.122 2.12l-5.657 5.658Z"
    />
  </g>
</svg>`;
    buttonEl.appendChild(arrowEl);
  }

  const dropdownContent = document.createElement("div");
  dropdownContent.classList.add("dropdown-content");

  const dropdownList = document.createElement("ul");
  dropdownList.classList.add("dropdown-list");

  let optionEls = options.map((option) => {
    const listItem = document.createElement("li");
    listItem.classList.add("dropdown-list__item");
    listItem.dataset.value = option.value;
    listItem.dataset.selected = selected.value === option.value;
    listItem.textContent = option.label;
    const isDisabled = option.disabled === true;
    listItem.dataset.disabled = isDisabled;
    if (!isDisabled) {
      listItem.addEventListener("click", () => {
        selected = option;
        buttonContentEl.textContent = selected.label;
        optionEls.map((optionEl) => (optionEl.dataset.selected = false));
        listItem.dataset.selected = true;
        close();
        onSelect(option);
      });
    }

    return listItem;
  });

  dropdownList.append(...optionEls);

  let searchEl, separatorEl, searchInfoEl;
  if (search) {
    searchEl = document.createElement("input");
    searchEl.type = "search";
    searchEl.role = "searchbox";
    searchEl.placeholder = "Search...";
    searchEl.classList.add("search", "dropdown-search");
    searchEl.addEventListener("input", (e) => {
      const query = e.target.value.toLowerCase();
      const passedOptions = options.filter((option) =>
        option.label.toLowerCase().includes(query)
      );
      searchInfoEl.hidden = !!passedOptions.length;
      optionEls.map((optionEl) => {
        optionEl.hidden = !passedOptions.find(
          (option) => option.value === optionEl.dataset.value
        );
      });
    });

    separatorEl = document.createElement("span");
    separatorEl.classList.add("dropdown-separator");

    searchInfoEl = document.createElement("span");
    searchInfoEl.classList.add("dropdown-search__info");
    searchInfoEl.hidden = true;
    searchInfoEl.textContent = "No results found.";
    dropdownList.prepend(searchInfoEl);

    dropdownContent.append(searchEl, separatorEl);
  }

  dropdownContent.appendChild(dropdownList);

  const dropdownPopupEl = createPopup(dropdownContent);
  dropdownPopupEl.classList.add("dropdown-popup");

  containerEl.append(titleEl, buttonEl, dropdownPopupEl);
  buttonEl.addEventListener("click", () => {
    opened ? close() : open();
  });

  return {
    containerEl,
    titleEl,
    buttonEl,
    buttonContentEl,
    arrowEl,
  };
}

const cards = {
  "languages-github": {
    label: "Languages (GitHub)",
    path: "top-langs/github",
    options: [
      {
        label: "Select username",
        query: "username",
        type: "input",
        value: DEFAULT_USERNAME,
      },
      {
        label: "Select theme",
        query: "theme",
        type: "dropdown",
        value: themes,
      },
      {
        label: "Select layout",
        query: "layout",
        type: "dropdown",
        value: layouts,
      },
    ],
  },
  "languages-wakatime": {
    label: "Languages (WakaTime)",
    path: "top-langs/wakatime",
    options: [
      {
        label: "Select username",
        query: "username",
        type: "input",
        value: "Toil",
      },
      {
        label: "Select theme",
        query: "theme",
        type: "dropdown",
        value: themes,
      },
      {
        label: "Select layout",
        query: "layout",
        type: "dropdown",
        value: layouts,
      },
    ],
  },
  "activity-github": {
    label: "Activity (GitHub)",
    path: "activity/github",
    options: [
      {
        label: "Select username",
        query: "username",
        type: "input",
        value: DEFAULT_USERNAME,
      },
      {
        label: "Select theme",
        query: "theme",
        type: "dropdown",
        value: themes,
      },
      {
        label: "Select period",
        query: "period",
        type: "dropdown",
        value: period,
      },
      {
        label: "Show title",
        query: "with_title",
        type: "checkbox",
        value: true,
      },
    ],
  },
};

const categories = Object.entries(cards).map(([key, val]) => ({
  value: key,
  disabled: !!val.disabled,
  ...val,
}));

let selectedCategory = categories[0];
const userData = new Map();
userData.set("username", DEFAULT_USERNAME);

function initCategory() {
  const generatorOptionsEl = document.querySelector(".generator-options");
  const category = selectedCategory.value;
  if (generatorOptionsEl.dataset.category === category) {
    return;
  }

  generatorOptionsEl.dataset.category = category;
  generatorOptionsEl.innerHTML = "";
  const optionEls = selectedCategory.options.map((option) => {
    const optionEl = document.createElement("li");
    optionEl.classList.add("generator_options__item");
    if (!userData.has(option.query)) {
      userData.set(
        option.query,
        Array.isArray(option.value)
          ? option.value.find((val) => !val.disabled)
          : option.value
      );
    }

    const selected = userData.get(option.query);
    switch (option.type) {
      case "dropdown": {
        const dropdown = createDropdown({
          title: option.label,
          selected,
          search: !!option.search,
          options: option.value,
          onSelect: (opt) => {
            userData.set(option.query, opt);
          },
        });

        optionEl.appendChild(dropdown.containerEl);
        break;
      }
      case "input": {
        const labelEl = document.createElement("label");
        const id = `generator-${option.query}`;
        labelEl.setAttribute("for", id);
        labelEl.classList.add("textfield-wrapper");

        let timer;
        const inputEl = document.createElement("input");
        inputEl.type = "text";
        inputEl.placeholder = option.value;
        inputEl.value = selected;
        inputEl.id = id;
        inputEl.classList.add("textfield", "textfield_outline");
        inputEl.addEventListener("input", () => {
          clearTimeout(timer);
          timer = setTimeout(() => {
            userData.set(option.query, inputEl.value);
          }, 100);
        });

        labelEl.append(inputEl, option.label);
        optionEl.appendChild(labelEl);
        break;
      }
      case "checkbox": {
        const labelEl = document.createElement("label");
        labelEl.classList.add("checkbox-wrapper");
        const id = `generator-${option.query}`;
        labelEl.setAttribute("for", id);

        const inputEl = document.createElement("input");
        inputEl.type = "checkbox";
        inputEl.id = id;
        inputEl.checked = selected;
        inputEl.addEventListener("change", () => {
          userData.set(option.query, inputEl.checked);
        });
        labelEl.append(inputEl, option.label);
        optionEl.appendChild(labelEl);
        break;
      }
    }

    return optionEl;
  });

  generatorOptionsEl.append(...optionEls);
  updatePreview();
}

function updatePreview() {
  const generatedImg = document.getElementById("generated-image");
  const params = new URLSearchParams(
    selectedCategory.options.reduce((result, option) => {
      const data = userData.get(option.query);
      result[option.query] = typeof data === "object" ? data.value : data;
      return result;
    }, {})
  ).toString();
  generatedImg.src = `/v1/${selectedCategory.path}?${params}`;
}

function init() {
  const generatorCategoryEl = document.querySelector(".generator-category");
  const generatorButtonEl = document.querySelector(".generator-button");

  const categoryDropdown = createDropdown({
    title: "Select stats category",
    selected: selectedCategory,
    search: true,
    options: categories,
    onSelect: (option) => {
      selectedCategory = option;
      initCategory(selectedCategory);
    },
  });

  generatorCategoryEl.appendChild(categoryDropdown.containerEl);

  initCategory();

  generatorButtonEl.addEventListener("click", () => {
    updatePreview();
  });
  window.addEventListener("keypress", (e) => {
    if (e.key !== "Enter") {
      return;
    }

    updatePreview();
  });
}

init();
