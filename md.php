<?php
/**
 * md.php
 * 
 * Lists and links all markdown files in the project, excluding node_modules.
 */


// Async execute mdbom command (non-blocking)
// Uses exec() with background execution (&) and output redirection
if (function_exists('exec')) {
    $homeDir = $_SERVER['HOME'] ?? getenv('HOME') ?? '~';
    $mdbomScript = $homeDir . '/scripts/convert-md-to-utf8bom.sh';
    
    // Check if script exists, then run asynchronously in background
    if (file_exists($mdbomScript)) {
        // Run in background: & at end, redirect output to /dev/null
        // This doesn't block PHP execution
        exec("bash {$mdbomScript} > /dev/null 2>&1 &");
    }
}

$rootDir = __DIR__;
$excludeDirs = ['node_modules', '.git', 'vendor', 'builds'];

function findMarkdownFiles($dir, $rootDir, $excludeDirs) {
    $files = [];
    $items = scandir($dir);
    
    foreach ($items as $item) {
        if ($item === '.' || $item === '..') continue; 
        
        $path = $dir . DIRECTORY_SEPARATOR . $item;
        
        if (is_dir($path)) {
            if (!in_array($item, $excludeDirs)) {
                $files = array_merge($files, findMarkdownFiles($path, $rootDir, $excludeDirs));
            }
        } elseif (pathinfo($item, PATHINFO_EXTENSION) === 'md') {
            $relativePath = str_replace($rootDir . DIRECTORY_SEPARATOR, '', $path);
            $files[] = $relativePath;
        }
    }
    
    return $files;
}

$mdFiles = findMarkdownFiles($rootDir, $rootDir, $excludeDirs);

$sortByDepthThenAlpha = $mdFiles;

// Define core files list in the order they should appear
$coreFiles = [
    'readme.md',
    'position.md',
    'position-erp.md',
    'prd.md',
    'architecture.md',
    'compliance.md',
    'marketing/planner.md',
    'markets/readme.md',
    'apps/backend/readme.md',
    'apps/spa/readme.md',
    'apps/backend/design-docs/architecture.md',
    'apps/backend/design-docs/modules/api.md',
    'apps/spa/design-docs/ui.md',
    'apps/spa/design-docs/api.md',
    'dev-primer/readme.md'
];

// Normalize paths for comparison (handle both / and \ separators)
function normalizePath($path) {
    return str_replace('\\', '/', strtolower($path));
}

$coreFilesNormalized = array_map('normalizePath', $coreFiles);

// Filter and order core files according to the specified order
$coreFilesList = [];
foreach ($coreFiles as $coreFile) {
    $normalizedCore = normalizePath($coreFile);
    foreach ($mdFiles as $file) {
        if (normalizePath($file) === $normalizedCore) {
            $coreFilesList[] = $file;
            break;
        }
    }
}

$todoFiles = array_filter($mdFiles, fn($f) => strcasecmp(basename($f), 'TODO.md') === 0);
$otherFiles = array_filter($mdFiles, function($f) use ($coreFilesNormalized) {
    $normalized = normalizePath($f);
    return !in_array($normalized, $coreFilesNormalized) && 
           strcasecmp(basename($f), 'TODO.md') !== 0;
});

$coreFilesList = array_values($coreFilesList);
$todoFiles = array_values($todoFiles);
$otherFiles = array_values($otherFiles);

// usort($readmeFiles, $sortByDepthThenAlpha);
// usort($todoFiles, $sortByDepthThenAlpha);
// usort($otherFiles, $sortByDepthThenAlpha);
?>
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Markdown Files</title>
    <style>
        * { box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            max-width: 1100px;
            margin: 0 auto;
            padding: 2rem;
            background: #ffffff;
            color: #24292f;
        }
        h1 {
            color: #0969da;
            border-bottom: 1px solid #d0d7de;
            padding-bottom: 0.5rem;
        }
        h2 {
            color: #24292f;
            font-size: 1.1rem;
            margin-top: 0;
            margin-bottom: 0.5rem;
        }
        .count {
            color: #57606a;
            font-size: 0.9rem;
            margin-bottom: 1rem;
        }
        .search-wrap {
            position: sticky;
            top: 0;
            background: #ffffff;
            padding: 1rem 0;
            margin: -1rem 0 0.5rem;
            z-index: 10;
        }
        .search-box {
            width: 100%;
            padding: 0.6rem 1rem;
            font-size: 1rem;
            border: 1px solid #d0d7de;
            border-radius: 6px;
        }
        .search-box:focus {
            outline: none;
            border-color: #0969da;
            box-shadow: 0 0 0 3px rgba(9, 105, 218, 0.15);
        }
        .top-sections {
            display: flex;
            gap: 2rem;
            margin-bottom: 1.5rem;
        }
        .top-sections .section {
            flex: 1;
            background: #f6f8fa;
            border: 1px solid #d0d7de;
            border-radius: 6px;
            padding: 1rem;
        }
        ul {
            list-style: none;
            padding: 0;
            margin: 0;
        }
        li {
            margin: 0.4rem 0;
        }
        li.hidden {
            display: none;
        }
        a {
            color: #0969da;
            text-decoration: none;
            padding: 0.3rem 0.5rem;
            border-radius: 4px;
            display: inline-block;
            transition: background 0.15s;
        }
        a:hover {
            background: rgba(9, 105, 218, 0.1);
            text-decoration: underline;
        }
        .folder {
            color: #57606a;
        }
        .filename {
            color: #24292f;
        }
        .section {
            margin-bottom: 1.5rem;
        }
        .section.hidden {
            display: none;
        }
        .no-results {
            color: #57606a;
            font-style: italic;
            display: none;
            padding: 1rem 0;
        }
        @media (max-width: 700px) {
            .top-sections {
                flex-direction: column;
            }
        }
    </style>
</head>
<body>
    <h1>📄 Markdown Files</h1>
    <p class="count">Found <?= count($mdFiles) ?> markdown files</p>
    
    <div class="search-wrap">
        <input type="text" class="search-box" id="search" placeholder="Search files..." autofocus>
    </div>
    
    <div class="top-sections">
        <div class="section" id="readme-section">
            <h2>⭐ Core Files (<?= count($coreFilesList) ?>)</h2>
            <ul id="readme-list">
                <?php foreach ($coreFilesList as $file): ?>
                    <?php
                        $parts = explode(DIRECTORY_SEPARATOR, $file);
                        $filename = array_pop($parts);
                        $folder = implode('/', $parts);
                    ?>
                    <li data-path="<?= htmlspecialchars(strtolower($file)) ?>" data-filename="<?= htmlspecialchars(strtolower($filename)) ?>" data-folder="<?= htmlspecialchars(strtolower($folder)) ?>">
                        <a href="<?= htmlspecialchars($file) ?>" target="_blank">
                            <?php if ($folder): ?>
                                <span class="folder"><?= htmlspecialchars($folder) ?>/</span>
                            <?php endif; ?>
                            <span class="filename"><?= htmlspecialchars($filename) ?></span>
                        </a>
                    </li>
                <?php endforeach; ?>
            </ul>
        </div>

        <div class="section" id="todo-section">
            <h2>✅ TODO (<?= count($todoFiles) ?>)</h2>
            <ul id="todo-list">
                <?php foreach ($todoFiles as $file): ?>
                    <?php
                        $parts = explode(DIRECTORY_SEPARATOR, $file);
                        $filename = array_pop($parts);
                        $folder = implode('/', $parts);
                    ?>
                    <li data-path="<?= htmlspecialchars(strtolower($file)) ?>" data-filename="<?= htmlspecialchars(strtolower($filename)) ?>" data-folder="<?= htmlspecialchars(strtolower($folder)) ?>">
                        <a href="<?= htmlspecialchars($file) ?>" target="_blank">
                            <?php if ($folder): ?>
                                <span class="folder"><?= htmlspecialchars($folder) ?>/</span>
                            <?php endif; ?>
                            <span class="filename"><?= htmlspecialchars($filename) ?></span>
                        </a>
                    </li>
                <?php endforeach; ?>
            </ul>
        </div>
    </div>

    <div class="section" id="other-section">
        <h2>📝 Other Markdown Files (<?= count($otherFiles) ?>)</h2>
        <ul id="other-list">
            <?php foreach ($otherFiles as $file): ?>
                <?php
                    $parts = explode(DIRECTORY_SEPARATOR, $file);
                    $filename = array_pop($parts);
                    $folder = implode('/', $parts);
                ?>
                <li data-path="<?= htmlspecialchars(strtolower($file)) ?>" data-filename="<?= htmlspecialchars(strtolower($filename)) ?>" data-folder="<?= htmlspecialchars(strtolower($folder)) ?>">
                    <a href="<?= htmlspecialchars($file) ?>" target="_blank">
                        <?php if ($folder): ?>
                            <span class="folder"><?= htmlspecialchars($folder) ?>/</span>
                        <?php endif; ?>
                        <span class="filename"><?= htmlspecialchars($filename) ?></span>
                    </a>
                </li>
            <?php endforeach; ?>
        </ul>
    </div>

    <p class="no-results" id="no-results">No files match your search.</p>

    <script>
        const search = document.getElementById('search');
        const readmeSection = document.getElementById('readme-section');
        const todoSection = document.getElementById('todo-section');
        const otherSection = document.getElementById('other-section');
        const readmeItems = document.querySelectorAll('#readme-list li');
        const todoItems = document.querySelectorAll('#todo-list li');
        const otherItems = document.querySelectorAll('#other-list li');
        const noResults = document.getElementById('no-results');

        // Tokenize search query and check if all words appear in the path
        // This is faster and more flexible than regex - matches "api chats" in "api.chats.md" or "chats/api.md"
        function matchesQuery(path, query) {
            if (!query) return true;
            
            // Split query into words (tokens)
            const words = query.toLowerCase().trim().split(/\s+/).filter(w => w.length > 0);
            if (words.length === 0) return true;
            
            const pathLower = path.toLowerCase();
            
            // All words must appear somewhere in the path
            return words.every(word => pathLower.includes(word));
        }

        // Add "/" shortcut to focus search input
        document.addEventListener('keydown', function(e) {
            // Only trigger if not typing in an input/textarea and "/" is pressed
            if (e.key === '/' && e.target.tagName !== 'INPUT' && e.target.tagName !== 'TEXTAREA') {
                e.preventDefault();
                search.focus();
            }
        });

        search.addEventListener('input', function() {
            const query = this.value.trim();
            let readmeVisible = 0;
            let todoVisible = 0;
            let otherVisible = 0;

            // If empty query, show all items
            if (!query) {
                readmeItems.forEach(item => item.classList.remove('hidden'));
                todoItems.forEach(item => item.classList.remove('hidden'));
                otherItems.forEach(item => item.classList.remove('hidden'));
                readmeSection.classList.remove('hidden');
                todoSection.classList.remove('hidden');
                otherSection.classList.remove('hidden');
                noResults.style.display = 'none';
                return;
            }

            // Search readme items
            readmeItems.forEach(item => {
                const path = item.dataset.path;
                const match = matchesQuery(path, query);
                item.classList.toggle('hidden', !match);
                if (match) readmeVisible++;
            });

            // Search todo items
            todoItems.forEach(item => {
                const path = item.dataset.path;
                const match = matchesQuery(path, query);
                item.classList.toggle('hidden', !match);
                if (match) todoVisible++;
            });

            // Search other items
            otherItems.forEach(item => {
                const path = item.dataset.path;
                const match = matchesQuery(path, query);
                item.classList.toggle('hidden', !match);
                if (match) otherVisible++;
            });

            // Update section visibility
            readmeSection.classList.toggle('hidden', readmeVisible === 0);
            todoSection.classList.toggle('hidden', todoVisible === 0);
            otherSection.classList.toggle('hidden', otherVisible === 0);
            noResults.style.display = (readmeVisible === 0 && todoVisible === 0 && otherVisible === 0) ? 'block' : 'none';
        });
    </script>
</body>
</html>
