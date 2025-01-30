import { AtrosStep, PredefinedParameters } from "../.atros";
const { apt_get, cask, yay } = PredefinedParameters;

// In Atros, creating your automated setup in a cross-platform way is the priority
// Currently we support MacOS and every distribution that's based on Arch, Debian & Fedora Linux

// Since this is the priority, you can define a system for almost anything:
// Tasks, packages and parameters can be system-specific
// Let's create an example step for each one of these

const step: AtrosStep = {
  title: "Hello World Step",
  tasks: [
    {
      type: "shell",
      cmd: "echo 'Hey, welcome to Atros'",
      // It's that simple to create a task for a specific system
      system: "debian",
    },
    {
      type: "install",
      packages: ["package-names", "that-you", "want-to-install"],
      // This field is also valid for install tasks
      system: "arch",
    },
    {
      type: "install",
      packages: [
        "cross-platform-package",
        {
          // You can define specific packages in a cross-platform task
          // to work only in one system
          system: "fedora",
          list: ["some-arch-package", "in-a-cross-platform-task"],
        },
        // This way, you don't need to create verbose tasks
      ],
    },
    {
      type: "install",
      packages: ["cross-platform-package"],
      params: [
        "--cross_platform_param",
        {
          // And just like package lists, you can define system specific parameters
          // This way you can control each package manager in each system inside a task
          // Pre-defined parameters are created using this features
          // Go to their definition and take a look if you're curious
          system: "mac",
          list: ["--params", "--for", "--brew", "--like", "--cask"],
          // Spoiler: What if you want to install neovide, which uses --cask parameter?
          // You would need to create another task to only install a package
          // Or would you? You can also define params for this list, just like you define system
          // To see it in action, check out the next example
        },
      ],
    },
  ],
};

export default step;
