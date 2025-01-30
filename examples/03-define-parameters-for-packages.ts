import { AtrosStep, PredefinedParameters } from "../.atros";
const { apt_get, cask, yay } = PredefinedParameters;

const step: AtrosStep = {
  title: "Hello World Step",
  tasks: [
    {
      type: "install",
      system: "arch",
      packages: ["regular-package", "a-package-that-should-use-yay"],
      // What happens now? Do you have to create a new task
      // Only to specify that it uses prebuilt yay parameter?
      // There's a better way
    },
    {
      type: "install",
      system: "arch",
      packages: [
        "regular-package",
        {
          // Just like systems, you can define parameters for a list of package
          params: [yay],
          list: ["a-package-that-should-use-yay"],
        },
      ],
    },

    // So you don't have to create more tasks just to define parameters.
    // Which would be verbose, trust me. I was there before we had this feature
  ],
};

export default step;
