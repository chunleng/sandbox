variable "stage" {
  type        = string
  description = "The Digital Ocean stage of the project to deploy to. i.e. Production, Staging, Development"
  validation {
    condition     = contains(["Production", "Staging", "Development"], var.stage)
    error_message = "var.stage can only be \"Production\", \"Staging\" or \"Development\""
  }
}

variable "project" {
  type        = string
  description = "Name of the project used for deployment"
}

variable "default_region" {
  type        = string
  description = "Default region to deploy the infrastructure"
}
