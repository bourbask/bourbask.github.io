/**
 * Contact Form Manager
 * Handles contact form submission and validation
 */

import { CONFIG } from "../config/settings.js";

class ContactFormManager {
  constructor(i18nManager) {
    this.i18n = i18nManager;
    this.form = document.querySelector(".contact-form");
  }

  /**
   * Initialize contact form
   */
  init() {
    if (this.form) {
      this.setupEventListeners();
    }
  }

  /**
   * Validate form data
   */
  validateForm(formData) {
    const name =
      formData.get("name") ||
      this.form.querySelector('input[type="text"]').value;
    const email =
      formData.get("email") ||
      this.form.querySelector('input[type="email"]').value;
    const message =
      formData.get("message") || this.form.querySelector("textarea").value;

    if (!name || !email || !message) {
      alert(this.i18n.t("fillAllFields"));
      return false;
    }

    return { name, email, message };
  }

  /**
   * Handle form submission
   */
  handleSubmit(e) {
    e.preventDefault();

    const formData = new FormData(this.form);
    const validatedData = this.validateForm(formData);

    if (!validatedData) return;

    // Create mailto link
    const subject = encodeURIComponent(
      `Portfolio Contact from ${validatedData.name}`
    );
    const body = encodeURIComponent(
      `Name: ${validatedData.name}\nEmail: ${validatedData.email}\n\nMessage:\n${validatedData.message}`
    );
    const mailtoLink = `mailto:${CONFIG.CONTACT_EMAIL}?subject=${subject}&body=${body}`;

    // Open email client
    window.location.href = mailtoLink;

    // Show success message
    alert(this.i18n.t("emailClientOpened"));

    // Reset form
    this.form.reset();
  }

  /**
   * Setup event listeners
   */
  setupEventListeners() {
    this.form.addEventListener("submit", (e) => this.handleSubmit(e));
  }
}

export default ContactFormManager;
