// Pa11y beforeScript: force dark theme before accessibility audit
// Used by quality.yml and make a11y-dark
module.exports = async (page) => {
    await page.evaluate(() => {
        document.documentElement.setAttribute('data-theme', 'dark');
        localStorage.setItem('theme', 'dark');
    });
    // Brief pause for theme transition to settle
    await new Promise(resolve => setTimeout(resolve, 300));
};
