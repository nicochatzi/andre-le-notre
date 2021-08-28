const project = require('./lib/project');
const log = require('./lib/log');
const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

const getLogs = (subDir) => {
  const logDir = path.join(project.dirs.logs, subDir);

  if (!fs.existsSync(logDir)) {
    fs.mkdirSync(logDir, { recursive: true });
  }

  return {
    stdoutLog: path.join(logDir, `stderr.log`),
    stderrLog: path.join(logDir, `stdout.log`),
  };
};

const runApp = (subDir) => {
  const { stdoutLog, stderrLog } = getLogs(subDir);

  execSync('cargo run --release', {
    detached: false,
    stdio: [
      'ignore',
      fs.openSync(stdoutLog, 'w+'),
      fs.openSync(stderrLog, 'w+'),
    ],
    cwd: project.dirs.root,
  });
};

const watchDogRunnerWithRateLimitInSeconds = (rerunLimitSecs) => {
  const timestamp = `${new Date().toISOString()}`;
  log.info(`Running app @ ${timestamp}`);

  try {
    runApp(timestamp);
  } catch (e) {
    const { stderrLog } = getLogs(timestamp);
    const errorLog = fs.readFileSync(stderrLog);

    log.error(
      `App crashed @ ${timestamp}
err : ${e}
file : ${stderrLog}
${errorLog}`
    );

    setTimeout(
      () => watchDogRunnerWithRateLimitInSeconds(rerunLimitSecs),
      rerunLimitSecs * 1000
    );
  }
};

watchDogRunnerWithRateLimitInSeconds(30);
