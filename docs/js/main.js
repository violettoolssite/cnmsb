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
        this.word = 'CNMSB';
        this.letters = [];
        this.targetPositions = [];
        this.phase = 'swirl'; // swirl, forming, formed, dissolving
        this.phaseTime = 0;
        this.animationId = null;
        
        // Timing (in frames, ~60fps)
        this.swirlDuration = 120;      // 2 seconds of chaos
        this.formDuration = 60;         // 1 second to form
        this.holdDuration = 90;         // 1.5 seconds holding
        this.dissolveDuration = 60;     // 1 second to dissolve
        
        // All possible letters for swirling
        this.alphabet = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%&*';
        this.numLetters = 50;
        
        if (this.canvas) {
            this.resize();
            window.addEventListener('resize', () => this.resize());
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
        
        // Create swirling letters
        for (let i = 0; i < this.numLetters; i++) {
            const angle = Math.random() * Math.PI * 2;
            const radius = 50 + Math.random() * 150;
            this.letters.push({
                char: this.alphabet[Math.floor(Math.random() * this.alphabet.length)],
                x: this.centerX + Math.cos(angle) * radius,
                y: this.centerY + Math.sin(angle) * radius,
                vx: (Math.random() - 0.5) * 4,
                vy: (Math.random() - 0.5) * 4,
                angle: Math.random() * Math.PI * 2,
                angularVel: (Math.random() - 0.5) * 0.2,
                size: 12 + Math.random() * 20,
                opacity: 0.3 + Math.random() * 0.7,
                targetX: 0,
                targetY: 0,
                isWordLetter: false,
                wordIndex: -1
            });
        }
        
        // Assign word letters
        const wordLetters = this.word.split('');
        const letterSpacing = 60;
        const totalWidth = (wordLetters.length - 1) * letterSpacing;
        const startX = this.centerX - totalWidth / 2;
        
        for (let i = 0; i < wordLetters.length; i++) {
            this.letters[i].char = wordLetters[i];
            this.letters[i].isWordLetter = true;
            this.letters[i].wordIndex = i;
            this.letters[i].targetX = startX + i * letterSpacing;
            this.letters[i].targetY = this.centerY;
            this.letters[i].size = 40;
        }
        
        this.animate();
    }
    
    animate() {
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
                    if (this.formedWord) {
                        this.formedWord.classList.add('visible');
                    }
                }
                break;
                
            case 'formed':
                this.updateFormed();
                if (this.phaseTime > this.holdDuration) {
                    this.phase = 'dissolving';
                    this.phaseTime = 0;
                    if (this.formedWord) {
                        this.formedWord.classList.remove('visible');
                        this.formedWord.classList.add('dissolving');
                    }
                }
                break;
                
            case 'dissolving':
                this.updateDissolving();
                if (this.phaseTime > this.dissolveDuration) {
                    this.phase = 'swirl';
                    this.phaseTime = 0;
                    if (this.formedWord) {
                        this.formedWord.classList.remove('dissolving');
                    }
                    // Randomize letters again
                    this.letters.forEach(letter => {
                        if (!letter.isWordLetter) {
                            letter.char = this.alphabet[Math.floor(Math.random() * this.alphabet.length)];
                        }
                        letter.vx = (Math.random() - 0.5) * 6;
                        letter.vy = (Math.random() - 0.5) * 6;
                    });
                }
                break;
        }
        
        this.draw();
        this.animationId = requestAnimationFrame(() => this.animate());
    }
    
    updateSwirl() {
        this.letters.forEach(letter => {
            // Orbital motion around center
            const dx = letter.x - this.centerX;
            const dy = letter.y - this.centerY;
            const dist = Math.sqrt(dx * dx + dy * dy);
            
            // Tangential velocity (swirl)
            const tangentX = -dy / dist;
            const tangentY = dx / dist;
            
            letter.vx += tangentX * 0.3;
            letter.vy += tangentY * 0.3;
            
            // Slight attraction to center
            letter.vx -= dx * 0.002;
            letter.vy -= dy * 0.002;
            
            // Damping
            letter.vx *= 0.98;
            letter.vy *= 0.98;
            
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
        const eased = this.easeOutCubic(progress);
        
        this.letters.forEach(letter => {
            if (letter.isWordLetter) {
                // Move toward target position
                letter.x += (letter.targetX - letter.x) * 0.15;
                letter.y += (letter.targetY - letter.y) * 0.15;
                letter.angle *= 0.9;
                letter.opacity = Math.min(1, letter.opacity + 0.05);
            } else {
                // Non-word letters fade and scatter
                letter.x += letter.vx;
                letter.y += letter.vy;
                letter.opacity = Math.max(0, 1 - eased);
            }
        });
    }
    
    updateFormed() {
        // Word letters stay in place with subtle float
        this.letters.forEach(letter => {
            if (letter.isWordLetter) {
                letter.x = letter.targetX + Math.sin(this.phaseTime * 0.05 + letter.wordIndex) * 2;
                letter.y = letter.targetY + Math.cos(this.phaseTime * 0.07 + letter.wordIndex) * 1;
                letter.opacity = 0; // Hide canvas letters, show CSS word
            }
        });
    }
    
    updateDissolving() {
        const progress = this.phaseTime / this.dissolveDuration;
        
        this.letters.forEach(letter => {
            if (letter.isWordLetter) {
                // Explode outward
                const angle = Math.random() * Math.PI * 2;
                letter.vx += Math.cos(angle) * 0.5;
                letter.vy += Math.sin(angle) * 0.5;
                letter.opacity = Math.min(1, progress);
            } else {
                // Fade back in
                letter.opacity = Math.min(0.7, progress);
            }
            
            letter.x += letter.vx;
            letter.y += letter.vy;
            letter.vx *= 0.98;
            letter.vy *= 0.98;
            letter.angle += letter.angularVel;
        });
    }
    
    draw() {
        this.letters.forEach(letter => {
            if (letter.opacity <= 0) return;
            
            this.ctx.save();
            this.ctx.translate(letter.x, letter.y);
            this.ctx.rotate(letter.angle);
            
            this.ctx.font = `bold ${letter.size}px 'JetBrains Mono', monospace`;
            this.ctx.textAlign = 'center';
            this.ctx.textBaseline = 'middle';
            
            // Glow effect for word letters
            if (letter.isWordLetter && this.phase !== 'swirl') {
                this.ctx.shadowColor = '#d4ff00';
                this.ctx.shadowBlur = 20;
            }
            
            this.ctx.fillStyle = `rgba(212, 255, 0, ${letter.opacity})`;
            this.ctx.fillText(letter.char, 0, 0);
            
            this.ctx.restore();
        });
    }
    
    easeOutCubic(t) {
        return 1 - Math.pow(1 - t, 3);
    }
    
    stop() {
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

