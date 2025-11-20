# Desktop Modules Backend Implementation

## Database Migration

Create file: `eiq-manager/migrations/YYYYMMDDHHMMSS_desktop_modules.sql`

```sql
-- Desktop Available Modules Configuration
CREATE TABLE IF NOT EXISTS desktop_modules (
    id SERIAL PRIMARY KEY,
    module_key VARCHAR(100) UNIQUE NOT NULL,
    module_name VARCHAR(255) NOT NULL,
    module_description TEXT,
    module_icon VARCHAR(50),
    module_category VARCHAR(50) NOT NULL,
    is_enabled BOOLEAN DEFAULT false,
    display_order INT DEFAULT 0,
    is_coming_soon BOOLEAN DEFAULT false,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Insert default modules
INSERT INTO desktop_modules (module_key, module_name, module_description, module_icon, module_category, is_enabled, display_order, is_coming_soon) VALUES
('email', 'Email', 'Full-featured email client with IMAP/SMTP support', 'mail', 'communication', true, 1, false),
('chat', 'Team Chat', 'Real-time messaging and collaboration', 'message-square', 'communication', true, 2, false),
('calendar', 'Calendar', 'Manage events and schedule meetings', 'calendar', 'productivity', false, 3, true),
('tasks', 'Tasks', 'Track and manage your to-do lists', 'check-square', 'productivity', false, 4, true),
('crm', 'CRM', 'Customer relationship management', 'users', 'business', false, 5, true),
('invoicing', 'Invoicing', 'Create and manage invoices', 'file-text', 'business', false, 6, true),
('settings', 'Settings', 'Configure your application preferences', 'settings', 'tools', true, 99, false)
ON CONFLICT (module_key) DO NOTHING;

-- Create indexes
CREATE INDEX IF NOT EXISTS idx_desktop_modules_enabled ON desktop_modules(is_enabled);
CREATE INDEX IF NOT EXISTS idx_desktop_modules_category ON desktop_modules(module_category);
```

## Symfony Entity

Create file: `eiq-manager/src/Entity/DesktopModule.php`

```php
<?php

namespace App\Entity;

use App\Repository\DesktopModuleRepository;
use Doctrine\ORM\Mapping as ORM;

#[ORM\Entity(repositoryClass: DesktopModuleRepository::class)]
#[ORM\Table(name: 'desktop_modules')]
class DesktopModule
{
    #[ORM\Id]
    #[ORM\GeneratedValue]
    #[ORM\Column]
    private ?int $id = null;

    #[ORM\Column(length: 100, unique: true)]
    private ?string $moduleKey = null;

    #[ORM\Column(length: 255)]
    private ?string $moduleName = null;

    #[ORM\Column(type: 'text', nullable: true)]
    private ?string $moduleDescription = null;

    #[ORM\Column(length: 50, nullable: true)]
    private ?string $moduleIcon = null;

    #[ORM\Column(length: 50)]
    private ?string $moduleCategory = null;

    #[ORM\Column(type: 'boolean')]
    private bool $isEnabled = false;

    #[ORM\Column(type: 'integer')]
    private int $displayOrder = 0;

    #[ORM\Column(type: 'boolean')]
    private bool $isComingSoon = false;

    #[ORM\Column(type: 'datetime')]
    private ?\DateTimeInterface $createdAt = null;

    #[ORM\Column(type: 'datetime')]
    private ?\DateTimeInterface $updatedAt = null;

    public function __construct()
    {
        $this->createdAt = new \DateTime();
        $this->updatedAt = new \DateTime();
    }

    // Getters and Setters

    public function getId(): ?int
    {
        return $this->id;
    }

    public function getModuleKey(): ?string
    {
        return $this->moduleKey;
    }

    public function setModuleKey(string $moduleKey): self
    {
        $this->moduleKey = $moduleKey;
        return $this;
    }

    public function getModuleName(): ?string
    {
        return $this->moduleName;
    }

    public function setModuleName(string $moduleName): self
    {
        $this->moduleName = $moduleName;
        return $this;
    }

    public function getModuleDescription(): ?string
    {
        return $this->moduleDescription;
    }

    public function setModuleDescription(?string $moduleDescription): self
    {
        $this->moduleDescription = $moduleDescription;
        return $this;
    }

    public function getModuleIcon(): ?string
    {
        return $this->moduleIcon;
    }

    public function setModuleIcon(?string $moduleIcon): self
    {
        $this->moduleIcon = $moduleIcon;
        return $this;
    }

    public function getModuleCategory(): ?string
    {
        return $this->moduleCategory;
    }

    public function setModuleCategory(string $moduleCategory): self
    {
        $this->moduleCategory = $moduleCategory;
        return $this;
    }

    public function isEnabled(): bool
    {
        return $this->isEnabled;
    }

    public function setIsEnabled(bool $isEnabled): self
    {
        $this->isEnabled = $isEnabled;
        return $this;
    }

    public function getDisplayOrder(): int
    {
        return $this->displayOrder;
    }

    public function setDisplayOrder(int $displayOrder): self
    {
        $this->displayOrder = $displayOrder;
        return $this;
    }

    public function isComingSoon(): bool
    {
        return $this->isComingSoon;
    }

    public function setIsComingSoon(bool $isComingSoon): self
    {
        $this->isComingSoon = $isComingSoon;
        return $this;
    }

    public function getCreatedAt(): ?\DateTimeInterface
    {
        return $this->createdAt;
    }

    public function setCreatedAt(\DateTimeInterface $createdAt): self
    {
        $this->createdAt = $createdAt;
        return $this;
    }

    public function getUpdatedAt(): ?\DateTimeInterface
    {
        return $this->updatedAt;
    }

    public function setUpdatedAt(\DateTimeInterface $updatedAt): self
    {
        $this->updatedAt = $updatedAt;
        return $this;
    }

    #[ORM\PreUpdate]
    public function updateTimestamp(): void
    {
        $this->updatedAt = new \DateTime();
    }
}
```

## Repository

Create file: `eiq-manager/src/Repository/DesktopModuleRepository.php`

```php
<?php

namespace App\Repository;

use App\Entity\DesktopModule;
use Doctrine\Bundle\DoctrineBundle\Repository\ServiceEntityRepository;
use Doctrine\Persistence\ManagerRegistry;

class DesktopModuleRepository extends ServiceEntityRepository
{
    public function __construct(ManagerRegistry $registry)
    {
        parent::__construct($registry, DesktopModule::class);
    }

    /**
     * Get all enabled modules ordered by display_order
     */
    public function findEnabledModules(): array
    {
        return $this->createQueryBuilder('m')
            ->where('m.isEnabled = :enabled')
            ->setParameter('enabled', true)
            ->orderBy('m.displayOrder', 'ASC')
            ->getQuery()
            ->getResult();
    }

    /**
     * Get all modules grouped by category
     */
    public function findAllGroupedByCategory(): array
    {
        $modules = $this->createQueryBuilder('m')
            ->orderBy('m.moduleCategory', 'ASC')
            ->addOrderBy('m.displayOrder', 'ASC')
            ->getQuery()
            ->getResult();

        $grouped = [];
        foreach ($modules as $module) {
            $category = $module->getModuleCategory();
            if (!isset($grouped[$category])) {
                $grouped[$category] = [];
            }
            $grouped[$category][] = $module;
        }

        return $grouped;
    }

    /**
     * Find module by key
     */
    public function findByKey(string $key): ?DesktopModule
    {
        return $this->findOneBy(['moduleKey' => $key]);
    }
}
```

## API Controller

Create file: `eiq-manager/src/Controller/Api/DesktopModulesApiController.php`

```php
<?php

namespace App\Controller\Api;

use App\Repository\DesktopModuleRepository;
use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\Routing\Annotation\Route;

#[Route('/api/v1/desktop/modules')]
class DesktopModulesApiController extends AbstractController
{
    public function __construct(
        private DesktopModuleRepository $moduleRepository
    ) {}

    /**
     * Get all enabled modules for desktop app
     */
    #[Route('', name: 'api_desktop_modules_list', methods: ['GET'])]
    public function listEnabledModules(): JsonResponse
    {
        $modules = $this->moduleRepository->findEnabledModules();

        $data = array_map(function($module) {
            return [
                'key' => $module->getModuleKey(),
                'name' => $module->getModuleName(),
                'description' => $module->getModuleDescription(),
                'icon' => $module->getModuleIcon(),
                'category' => $module->getModuleCategory(),
                'isComingSoon' => $module->isComingSoon(),
                'displayOrder' => $module->getDisplayOrder(),
            ];
        }, $modules);

        return new JsonResponse([
            'success' => true,
            'modules' => $data,
        ]);
    }

    /**
     * Get all modules grouped by category (for admin panel)
     */
    #[Route('/all', name: 'api_desktop_modules_all', methods: ['GET'])]
    public function listAllModules(): JsonResponse
    {
        $grouped = $this->moduleRepository->findAllGroupedByCategory();

        $data = [];
        foreach ($grouped as $category => $modules) {
            $data[$category] = array_map(function($module) {
                return [
                    'id' => $module->getId(),
                    'key' => $module->getModuleKey(),
                    'name' => $module->getModuleName(),
                    'description' => $module->getModuleDescription(),
                    'icon' => $module->getModuleIcon(),
                    'category' => $module->getModuleCategory(),
                    'isEnabled' => $module->isEnabled(),
                    'isComingSoon' => $module->isComingSoon(),
                    'displayOrder' => $module->getDisplayOrder(),
                ];
            }, $modules);
        }

        return new JsonResponse([
            'success' => true,
            'modules' => $data,
        ]);
    }
}
```

## Admin Controller (Twig)

Create file: `eiq-manager/src/Controller/Admin/DesktopModulesController.php`

```php
<?php

namespace App\Controller\Admin;

use App\Entity\DesktopModule;
use App\Repository\DesktopModuleRepository;
use Doctrine\ORM\EntityManagerInterface;
use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\Routing\Annotation\Route;

#[Route('/admin/desktop-modules')]
class DesktopModulesController extends AbstractController
{
    public function __construct(
        private DesktopModuleRepository $moduleRepository,
        private EntityManagerInterface $entityManager
    ) {}

    #[Route('', name: 'admin_desktop_modules_index')]
    public function index(): Response
    {
        $modules = $this->moduleRepository->findAllGroupedByCategory();

        return $this->render('admin/desktop_modules/index.html.twig', [
            'modules' => $modules,
        ]);
    }

    #[Route('/toggle/{id}', name: 'admin_desktop_modules_toggle', methods: ['POST'])]
    public function toggleModule(int $id): Response
    {
        $module = $this->moduleRepository->find($id);

        if (!$module) {
            $this->addFlash('error', 'Module not found');
            return $this->redirectToRoute('admin_desktop_modules_index');
        }

        $module->setIsEnabled(!$module->isEnabled());
        $this->entityManager->flush();

        $this->addFlash('success', sprintf(
            'Module "%s" has been %s',
            $module->getModuleName(),
            $module->isEnabled() ? 'enabled' : 'disabled'
        ));

        return $this->redirectToRoute('admin_desktop_modules_index');
    }

    #[Route('/update-order', name: 'admin_desktop_modules_update_order', methods: ['POST'])]
    public function updateOrder(Request $request): Response
    {
        $order = json_decode($request->getContent(), true);

        foreach ($order as $position => $moduleId) {
            $module = $this->moduleRepository->find($moduleId);
            if ($module) {
                $module->setDisplayOrder($position);
            }
        }

        $this->entityManager->flush();

        return $this->json(['success' => true]);
    }
}
```

## Twig Template

Create file: `eiq-manager/templates/admin/desktop_modules/index.html.twig`

```twig
{% extends 'base.html.twig' %}

{% block title %}Desktop Modules Configuration{% endblock %}

{% block body %}
<div class="container-fluid mt-4">
    <div class="row mb-4">
        <div class="col">
            <h1>Desktop Modules Configuration</h1>
            <p class="text-muted">Control which modules are available in the EvolveApp Desktop application</p>
        </div>
    </div>

    {% for flash_message in app.flashes('success') %}
        <div class="alert alert-success alert-dismissible fade show" role="alert">
            {{ flash_message }}
            <button type="button" class="btn-close" data-bs-dismiss="alert"></button>
        </div>
    {% endfor %}

    {% for flash_message in app.flashes('error') %}
        <div class="alert alert-danger alert-dismissible fade show" role="alert">
            {{ flash_message }}
            <button type="button" class="btn-close" data-bs-dismiss="alert"></button>
        </div>
    {% endfor %}

    {% for category, categoryModules in modules %}
        <div class="card mb-4">
            <div class="card-header">
                <h5 class="mb-0 text-capitalize">{{ category }}</h5>
            </div>
            <div class="card-body">
                <div class="table-responsive">
                    <table class="table table-hover">
                        <thead>
                            <tr>
                                <th>Icon</th>
                                <th>Module</th>
                                <th>Description</th>
                                <th>Status</th>
                                <th>Enabled</th>
                                <th>Actions</th>
                            </tr>
                        </thead>
                        <tbody>
                            {% for module in categoryModules %}
                                <tr>
                                    <td>
                                        <i class="bi bi-{{ module.moduleIcon }}"></i>
                                    </td>
                                    <td>
                                        <strong>{{ module.moduleName }}</strong>
                                    </td>
                                    <td>{{ module.moduleDescription }}</td>
                                    <td>
                                        {% if module.isComingSoon %}
                                            <span class="badge bg-secondary">Coming Soon</span>
                                        {% else %}
                                            <span class="badge bg-success">Available</span>
                                        {% endif %}
                                    </td>
                                    <td>
                                        {% if module.isEnabled %}
                                            <span class="badge bg-primary">Enabled</span>
                                        {% else %}
                                            <span class="badge bg-secondary">Disabled</span>
                                        {% endif %}
                                    </td>
                                    <td>
                                        <form method="post" action="{{ path('admin_desktop_modules_toggle', {id: module.id}) }}" style="display: inline;">
                                            <button type="submit" class="btn btn-sm {{ module.isEnabled ? 'btn-danger' : 'btn-primary' }}">
                                                {% if module.isEnabled %}
                                                    <i class="bi bi-toggle-on"></i> Disable
                                                {% else %}
                                                    <i class="bi bi-toggle-off"></i> Enable
                                                {% endif %}
                                            </button>
                                        </form>
                                    </td>
                                </tr>
                            {% endfor %}
                        </tbody>
                    </table>
                </div>
            </div>
        </div>
    {% endfor %}
</div>
{% endblock %}
```

## Usage

1. **Run migration:**
   ```bash
   psql -U your_user -d eiq_db -f migrations/YYYYMMDDHHMMSS_desktop_modules.sql
   ```

2. **Access admin panel:**
   - URL: `http://localhost:8547/admin/desktop-modules`
   - Toggle modules on/off
   - Desktop app will only show enabled modules

3. **API endpoint for desktop app:**
   - `GET /api/v1/desktop/modules` - Returns only enabled modules
   - `GET /api/v1/desktop/modules/all` - Returns all modules (admin only)

## Default Configuration

By default, these modules are **enabled**:
- ✅ Email
- ✅ Chat
- ✅ Settings

These are **disabled** (coming soon):
- ❌ Calendar
- ❌ Tasks
- ❌ CRM
- ❌ Invoicing
