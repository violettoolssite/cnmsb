/**
 * CNMSB - 命令行智能补全工具
 * Main JavaScript
 */

// ========================================
// Letter Swirl Animation
// ========================================

class LetterSwirl {
    constructor() {
        this.canvas = document.getElementById('letterCanvas');
        this.ctx = this.canvas ? this.canvas.getContext('2d') : null;
        this.formedWord = document.getElementById('formedWord');
        this.versionSelector = document.querySelector('.version-selector');
        this.word = 'CNMSB';
        this.letters = [];
        this.phase = 'swirl'; // swirl, forming, formed
        this.phaseTime = 0;
        this.animationId = null;
        this.isComplete = false;
        
        // Timing (in frames, ~60fps) - 更慢更丝滑
        this.swirlDuration = 180;       // 3 seconds of swirl
        this.formDuration = 120;        // 2 seconds to form (慢速聚合)
        
        // Only use the word letters for cleaner animation
        this.alphabet = 'CNMSB';
        this.numLetters = 30; // 减少字母数量
        
        if (this.canvas) {
            this.resize();
            window.addEventListener('resize', () => this.resize());
        }
        
        // 初始隐藏版本选择按钮
        if (this.versionSelector) {
            this.versionSelector.style.opacity = '0';
            this.versionSelector.style.transform = 'translateY(20px)';
            this.versionSelector.style.transition = 'opacity 0.8s ease, transform 0.8s ease';
        }
    }
    
    resize() {
        const dpr = window.devicePixelRatio || 1;
        const rect = this.canvas.parentElement.getBoundingClientRect();
        this.canvas.width = rect.width * dpr;
        this.canvas.height = rect.height * dpr;
        this.canvas.style.width = rect.width + 'px';
        this.canvas.style.height = rect.height + 'px';
        this.ctx.scale(dpr, dpr);
        this.width = rect.width;
        this.height = rect.height;
        this.centerX = this.width / 2;
        this.centerY = this.height / 2;
    }
    
    init() {
        if (!this.canvas || !this.ctx) return;
        
        // Create swirling letters - 只用 CNMSB 字母
        for (let i = 0; i < this.numLetters; i++) {
            const angle = Math.random() * Math.PI * 2;
            const radius = 80 + Math.random() * 200;
            this.letters.push({
                char: this.alphabet[Math.floor(Math.random() * this.alphabet.length)],
                x: this.centerX + Math.cos(angle) * radius,
                y: this.centerY + Math.sin(angle) * radius,
                vx: (Math.random() - 0.5) * 2, // 更慢的初始速度
                vy: (Math.random() - 0.5) * 2,
                angle: Math.random() * Math.PI * 2,
                angularVel: (Math.random() - 0.5) * 0.08, // 更慢的旋转
                size: 14 + Math.random() * 24,
                opacity: 0.2 + Math.random() * 0.5,
                targetX: 0,
                targetY: 0,
                isWordLetter: false,
                wordIndex: -1
            });
        }
        
        // Assign word letters (前5个是 CNMSB)
        const wordLetters = this.word.split('');
        const letterSpacing = 70;
        const totalWidth = (wordLetters.length - 1) * letterSpacing;
        const startX = this.centerX - totalWidth / 2;
        
        for (let i = 0; i < wordLetters.length; i++) {
            this.letters[i].char = wordLetters[i];
            this.letters[i].isWordLetter = true;
            this.letters[i].wordIndex = i;
            this.letters[i].targetX = startX + i * letterSpacing;
            this.letters[i].targetY = this.centerY;
            this.letters[i].size = 50;
            this.letters[i].opacity = 0.6;
        }
        
        this.animate();
    }
    
    animate() {
        if (this.isComplete) return;
        
        this.ctx.clearRect(0, 0, this.width, this.height);
        
        this.phaseTime++;
        
        switch (this.phase) {
            case 'swirl':
                this.updateSwirl();
                if (this.phaseTime > this.swirlDuration) {
                    this.phase = 'forming';
                    this.phaseTime = 0;
                }
                break;
                
            case 'forming':
                this.updateForming();
                if (this.phaseTime > this.formDuration) {
                    this.phase = 'formed';
                    this.phaseTime = 0;
                    this.onFormComplete();
                }
                break;
                
            case 'formed':
                // 保持静止，不再更新
                break;
        }
        
        this.draw();
        
        if (this.phase !== 'formed') {
            this.animationId = requestAnimationFrame(() => this.animate());
        }
    }
    
    onFormComplete() {
        this.isComplete = true;
        
        // 显示 CSS 文字
        if (this.formedWord) {
            this.formedWord.classList.add('visible');
        }
        
        // 隐藏 canvas 上的字母
        this.letters.forEach(letter => {
            if (letter.isWordLetter) {
                letter.opacity = 0;
            }
        });
        
        // 重新绘制（隐藏 word letters）
        this.ctx.clearRect(0, 0, this.width, this.height);
        this.draw();
        
        // 延迟显示版本选择按钮
        setTimeout(() => {
            if (this.versionSelector) {
                this.versionSelector.style.opacity = '1';
                this.versionSelector.style.transform = 'translateY(0)';
            }
        }, 300);
    }
    
    updateSwirl() {
        this.letters.forEach(letter => {
            // Orbital motion around center - 更平滑
            const dx = letter.x - this.centerX;
            const dy = letter.y - this.centerY;
            const dist = Math.sqrt(dx * dx + dy * dy) || 1;
            
            // Tangential velocity (swirl) - 更慢
            const tangentX = -dy / dist;
            const tangentY = dx / dist;
            
            letter.vx += tangentX * 0.15;
            letter.vy += tangentY * 0.15;
            
            // Slight attraction to center
            letter.vx -= dx * 0.001;
            letter.vy -= dy * 0.001;
            
            // Damping - 更平滑
            letter.vx *= 0.99;
            letter.vy *= 0.99;
            
            letter.x += letter.vx;
            letter.y += letter.vy;
            
            letter.angle += letter.angularVel;
            
            // Keep within bounds
            if (letter.x < 0 || letter.x > this.width) letter.vx *= -0.8;
            if (letter.y < 0 || letter.y > this.height) letter.vy *= -0.8;
        });
    }
    
    updateForming() {
        const progress = this.phaseTime / this.formDuration;
        const eased = this.easeInOutCubic(progress); // 更丝滑的缓动
        
        this.letters.forEach(letter => {
            if (letter.isWordLetter) {
                // 平滑移动到目标位置
                const ease = 0.05 + eased * 0.1; // 渐进加速
                letter.x += (letter.targetX - letter.x) * ease;
                letter.y += (letter.targetY - letter.y) * ease;
                letter.angle *= 0.95; // 慢慢停止旋转
                letter.opacity = Math.min(1, 0.6 + eased * 0.4);
                letter.size = 50 + eased * 10; // 轻微放大
            } else {
                // 非目标字母慢慢淡出
                letter.x += letter.vx * 0.5;
                letter.y += letter.vy * 0.5;
                letter.opacity = Math.max(0, letter.opacity - 0.015);
                letter.angle += letter.angularVel * 0.5;
            }
        });
    }
    
    draw() {
        this.letters.forEach(letter => {
            if (letter.opacity <= 0.01) return;
            
            this.ctx.save();
            this.ctx.translate(letter.x, letter.y);
            this.ctx.rotate(letter.angle);
            
            this.ctx.font = `bold ${letter.size}px 'JetBrains Mono', monospace`;
            this.ctx.textAlign = 'center';
            this.ctx.textBaseline = 'middle';
            
            // Glow effect for word letters during forming
            if (letter.isWordLetter && this.phase === 'forming') {
                this.ctx.shadowColor = '#d4ff00';
                this.ctx.shadowBlur = 15 * letter.opacity;
            }
            
            this.ctx.fillStyle = `rgba(212, 255, 0, ${letter.opacity})`;
            this.ctx.fillText(letter.char, 0, 0);
            
            this.ctx.restore();
        });
    }
    
    easeInOutCubic(t) {
        return t < 0.5 ? 4 * t * t * t : 1 - Math.pow(-2 * t + 2, 3) / 2;
    }
    
    stop() {
        this.isComplete = true;
        if (this.animationId) {
            cancelAnimationFrame(this.animationId);
        }
    }
}

// ========================================
// Loading Screen & Version Selection
// ========================================

class LoadingScreen {
    constructor() {
        this.screen = document.querySelector('.loading-screen');
        this.progressFill = document.querySelector('.progress-fill');
        this.progressText = document.querySelector('.progress-text');
        this.mainContent = document.querySelector('.main-content');
        this.nav = document.querySelector('nav');
        this.progress = 0;
        this.targetProgress = 0;
        this.isVersionSelected = false;
        this.letterSwirl = null;
    }

    init() {
        // 初始化字母动画
        this.letterSwirl = new LetterSwirl();
        this.letterSwirl.init();
        
        // 模拟加载进度
        this.simulateLoading();
        
        // 监听版本选择按钮
        const versionBtns = document.querySelectorAll('.version-btn');
        versionBtns.forEach(btn => {
            btn.addEventListener('click', (e) => this.handleVersionSelect(e));
        });
    }

    simulateLoading() {
        const stages = [
            { progress: 20, delay: 300 },
            { progress: 45, delay: 600 },
            { progress: 70, delay: 400 },
            { progress: 90, delay: 500 },
            { progress: 100, delay: 300 }
        ];

        let currentStage = 0;
        
        const runStage = () => {
            if (currentStage >= stages.length) return;
            
            const stage = stages[currentStage];
            this.targetProgress = stage.progress;
            this.animateProgress();
            
            currentStage++;
            setTimeout(runStage, stage.delay);
        };

        setTimeout(runStage, 500);
    }

    animateProgress() {
        const animate = () => {
            if (this.progress < this.targetProgress) {
                this.progress += 1;
                this.progressFill.style.width = `${this.progress}%`;
                this.progressText.textContent = `加载中 ${this.progress}%`;
                requestAnimationFrame(animate);
            }
        };
        animate();
    }

    handleVersionSelect(e) {
        e.preventDefault();
        if (this.isVersionSelected) return;
        this.isVersionSelected = true;

        const btn = e.currentTarget;
        const version = btn.dataset.version;

        // 保存版本选择
        localStorage.setItem('cnmsb-version', version);

        // 更新内容样式
        this.applyVersionStyle(version);

        // 按钮点击效果
        btn.style.transform = 'scale(0.95)';
        setTimeout(() => {
            btn.style.transform = '';
        }, 150);

        // 延迟后隐藏加载屏幕
        setTimeout(() => {
            this.hideLoadingScreen();
        }, 500);
    }

    applyVersionStyle(version) {
        document.body.dataset.version = version;
        
        // 更新文本内容
        const rudeElements = document.querySelectorAll('[data-rude]');
        const normalElements = document.querySelectorAll('[data-normal]');
        
        rudeElements.forEach(el => {
            if (version === 'rude') {
                el.style.display = '';
                el.style.visibility = 'visible';
            } else {
                el.style.display = 'none';
                el.style.visibility = 'hidden';
            }
        });
        
        normalElements.forEach(el => {
            if (version === 'normal') {
                el.style.display = '';
                el.style.visibility = 'visible';
            } else {
                el.style.display = 'none';
                el.style.visibility = 'hidden';
            }
        });

        // 更新版本切换链接
        const versionSwitch = document.querySelector('.version-switch');
        if (versionSwitch) {
            versionSwitch.textContent = version === 'rude' ? '切换正常版' : '切换脏话版';
            versionSwitch.dataset.currentVersion = version;
        }
    }

    hideLoadingScreen() {
        // 停止字母动画
        if (this.letterSwirl) {
            this.letterSwirl.stop();
        }
        
        this.screen.classList.add('hidden');
        this.mainContent.classList.add('visible');
        this.nav.classList.add('visible');
        document.body.classList.remove('loading');

        // 触发内容动画
        setTimeout(() => {
            this.initScrollAnimations();
        }, 300);
    }

    initScrollAnimations() {
        const animateElements = document.querySelectorAll('.animate-in');
        
        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    entry.target.classList.add('visible');
                }
            });
        }, {
            threshold: 0.1,
            rootMargin: '0px 0px -50px 0px'
        });

        animateElements.forEach(el => observer.observe(el));
    }
}

// ========================================
// Version Switcher
// ========================================

class VersionSwitcher {
    constructor() {
        this.switchBtn = document.querySelector('.version-switch');
    }

    init() {
        if (!this.switchBtn) return;

        this.switchBtn.addEventListener('click', () => {
            const currentVersion = this.switchBtn.dataset.currentVersion || 'rude';
            const newVersion = currentVersion === 'rude' ? 'normal' : 'rude';
            
            localStorage.setItem('cnmsb-version', newVersion);
            this.applyVersion(newVersion);
        });

        // 检查是否有保存的版本偏好
        const savedVersion = localStorage.getItem('cnmsb-version');
        if (savedVersion) {
            this.applyVersion(savedVersion);
            // 如果有保存的版本，跳过加载屏幕
            document.querySelector('.loading-screen')?.classList.add('hidden');
            document.querySelector('.main-content')?.classList.add('visible');
            document.querySelector('nav')?.classList.add('visible');
            document.body.classList.remove('loading');
        }
    }

    applyVersion(version) {
        document.body.dataset.version = version;
        
        const rudeElements = document.querySelectorAll('[data-rude]');
        const normalElements = document.querySelectorAll('[data-normal]');
        
        rudeElements.forEach(el => {
            if (version === 'rude') {
                el.style.display = '';
                el.style.visibility = 'visible';
            } else {
                el.style.display = 'none';
                el.style.visibility = 'hidden';
            }
        });
        
        normalElements.forEach(el => {
            if (version === 'normal') {
                el.style.display = '';
                el.style.visibility = 'visible';
            } else {
                el.style.display = 'none';
                el.style.visibility = 'hidden';
            }
        });

        if (this.switchBtn) {
            this.switchBtn.textContent = version === 'rude' ? '切换正常版' : '切换脏话版';
            this.switchBtn.dataset.currentVersion = version;
        }
    }
}

// ========================================
// Copy Code Functionality
// ========================================

function copyCode(btn) {
    const codeBlock = btn.parentElement;
    const code = codeBlock.querySelector('pre').innerText;
    
    navigator.clipboard.writeText(code).then(() => {
        const originalText = btn.textContent;
        btn.textContent = '已复制';
        btn.style.background = 'var(--accent)';
        btn.style.color = 'var(--bg)';
        btn.style.borderColor = 'var(--accent)';
        
        setTimeout(() => {
            btn.textContent = originalText;
            btn.style.background = '';
            btn.style.color = '';
            btn.style.borderColor = '';
        }, 2000);
    });
}

// ========================================
// Smooth Scroll for Navigation
// ========================================

function initSmoothScroll() {
    document.querySelectorAll('a[href^="#"]').forEach(anchor => {
        anchor.addEventListener('click', function(e) {
            e.preventDefault();
            const target = document.querySelector(this.getAttribute('href'));
            if (target) {
                target.scrollIntoView({
                    behavior: 'smooth',
                    block: 'start'
                });
            }
        });
    });
}

// ========================================
// Scroll Animations
// ========================================

function initScrollAnimations() {
    const observerOptions = {
        threshold: 0.1,
        rootMargin: '0px 0px -50px 0px'
    };

    const observer = new IntersectionObserver((entries) => {
        entries.forEach((entry, index) => {
            if (entry.isIntersecting) {
                // 添加延迟以创建级联效果
                setTimeout(() => {
                    entry.target.classList.add('visible');
                }, index * 50);
            }
        });
    }, observerOptions);

    // 观察所有需要动画的元素
    const animateElements = document.querySelectorAll('.feature-item, .command-category, .shortcut-item, .animate-in');
    animateElements.forEach(el => {
        el.classList.add('animate-in');
        observer.observe(el);
    });
}

// ========================================
// Terminal Animation
// ========================================

function initTerminalAnimation() {
    const terminals = document.querySelectorAll('.terminal-showcase');
    
    terminals.forEach(terminal => {
        const lines = terminal.querySelectorAll('.terminal-line');
        const menus = terminal.querySelectorAll('.terminal-menu');
        
        // 初始隐藏
        lines.forEach((line, i) => {
            line.style.opacity = '0';
            line.style.transform = 'translateX(-10px)';
        });
        
        menus.forEach(menu => {
            menu.style.opacity = '0';
            menu.style.transform = 'translateX(-10px)';
        });
        
        // 观察终端进入视口
        const observer = new IntersectionObserver((entries) => {
            entries.forEach(entry => {
                if (entry.isIntersecting) {
                    animateTerminal(lines, menus);
                    observer.unobserve(entry.target);
                }
            });
        }, { threshold: 0.3 });
        
        observer.observe(terminal);
    });
}

function animateTerminal(lines, menus) {
    let delay = 0;
    
    lines.forEach((line, i) => {
        setTimeout(() => {
            line.style.transition = 'opacity 0.4s ease, transform 0.4s ease';
            line.style.opacity = '1';
            line.style.transform = 'translateX(0)';
        }, delay);
        delay += 200;
        
        // 如果下一个元素是菜单，添加菜单动画
        if (i < menus.length) {
            setTimeout(() => {
                menus[i].style.transition = 'opacity 0.4s ease, transform 0.4s ease';
                menus[i].style.opacity = '1';
                menus[i].style.transform = 'translateX(0)';
            }, delay);
            delay += 300;
        }
    });
}

// ========================================
// Parallax Effect for Hero
// ========================================

function initParallax() {
    const hero = document.querySelector('.hero');
    if (!hero) return;

    window.addEventListener('scroll', () => {
        const scrolled = window.pageYOffset;
        const heroContent = hero.querySelector('.hero-content');
        
        if (heroContent && scrolled < window.innerHeight) {
            heroContent.style.transform = `translateY(${scrolled * 0.3}px)`;
            heroContent.style.opacity = 1 - (scrolled / window.innerHeight);
        }
    });
}

// ========================================
// Feature Card Hover Effects
// ========================================

function initFeatureCardEffects() {
    const cards = document.querySelectorAll('.feature-item');
    
    cards.forEach(card => {
        card.addEventListener('mouseenter', function() {
            const number = this.querySelector('.feature-number');
            if (number) {
                number.style.transform = 'translateX(5px)';
                number.style.transition = 'transform 0.3s ease';
            }
        });
        
        card.addEventListener('mouseleave', function() {
            const number = this.querySelector('.feature-number');
            if (number) {
                number.style.transform = 'translateX(0)';
            }
        });
    });
}

// ========================================
// Typing Effect for Hero
// ========================================

function initTypingEffect() {
    const projectName = document.querySelector('.project-name');
    if (!projectName) return;

    const text = projectName.textContent;
    projectName.textContent = '';
    projectName.style.borderRight = '3px solid var(--accent)';
    
    let i = 0;
    const typeWriter = () => {
        if (i < text.length) {
            projectName.textContent += text.charAt(i);
            i++;
            setTimeout(typeWriter, 150);
        } else {
            // 闪烁光标效果
            let visible = true;
            setInterval(() => {
                projectName.style.borderRight = visible ? '3px solid var(--accent)' : '3px solid transparent';
                visible = !visible;
            }, 500);
        }
    };

    // 延迟开始打字效果
    setTimeout(typeWriter, 500);
}

// ========================================
// Initialize Everything
// ========================================

document.addEventListener('DOMContentLoaded', () => {
    // 初始化加载屏幕
    const loadingScreen = new LoadingScreen();
    const versionSwitcher = new VersionSwitcher();

    // 检查是否有保存的版本
    const savedVersion = localStorage.getItem('cnmsb-version');
    
    if (savedVersion) {
        // 有保存的版本，直接显示内容
        versionSwitcher.init();
        initAfterLoad();
    } else {
        // 没有保存的版本，显示加载屏幕
        document.body.classList.add('loading');
        loadingScreen.init();
        versionSwitcher.init();
        
        // 等待版本选择后初始化其他功能
        const observer = new MutationObserver((mutations) => {
            mutations.forEach(mutation => {
                if (mutation.target.classList.contains('hidden')) {
                    initAfterLoad();
                    observer.disconnect();
                }
            });
        });
        
        observer.observe(document.querySelector('.loading-screen'), {
            attributes: true,
            attributeFilter: ['class']
        });
    }
});

function initAfterLoad() {
    initSmoothScroll();
    initScrollAnimations();
    initTerminalAnimation();
    initParallax();
    initFeatureCardEffects();
    // initTypingEffect(); // 如果需要打字效果，取消注释
}

// 将 copyCode 暴露到全局作用域
window.copyCode = copyCode;

