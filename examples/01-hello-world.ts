// Let's get the important stuff out of our .atros file
import { AtrosStep, PredefinedParameters } from "../.atros";

// You can use these pre-defined parameters for your install tasks
// to use them, put them inside the list of your params field
const { apt_get, cask, yay } = PredefinedParameters;

const step: AtrosStep = {
  title: "Hello World Step",
  tasks: [
    {
      // We have two kinds of tasks: Shell and Install
      type: "shell",
      // Shell task takes a cmd field, later when we run our task
      // this field's string value will be executed using `sh` command
      cmd: "echo 'Hey, welcome to Atros'",
      // Later we'll see that we can define system for every task
      // For example, we can say that this task will only work on Arch Linux
    },
    {
      // This is how you define an Install task
      type: "install",
      // And install task takes a packages field, which is a list
      // This list doesn't only accept strings, but also objects.
      // We'll take a closer look at these objects in other example files.
      packages: ["package-names", "that-you", "want-to-install"],
      // You can optinally give parameters to your system's package managers.
      // Just like packages, this list also accepts objects
      // And again, we'll take a look that object
      params: ["--parameter-name"],
    },
    {
      type: "install",
      packages: ["package-installed", "wity-yay"],
      // You can also use pre-defined parameters
      // To see what that parameter will do, and the system it'll work on
      // Hover over that parameter
      params: [yay],
    },
  ],
};

export default step;
