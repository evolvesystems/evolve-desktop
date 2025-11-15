# Symfony API Implementation Guide

**Framework**: Symfony 7.3
**Database**: PostgreSQL 17
**Authentication**: JWT (LexikJWT Bundle)
**Target**: eiq-manager repository
**Date**: 2025-11-14

---

## 📋 Table of Contents

1. [Initial Setup](#initial-setup)
2. [CRM Contact API Controller](#crm-contact-api-controller)
3. [Email API Controller](#email-api-controller)
4. [Calendar API Controller](#calendar-api-controller)
5. [Authentication & Security](#authentication--security)
6. [Testing](#testing)

---

## 🚀 Initial Setup

### 1. Install Dependencies

```bash
cd /home/john/sources/eiq-manager

# JWT Authentication
composer require lexik/jwt-authentication-bundle

# CORS Support
composer require nelmio/cors-bundle

# API Platform (optional - for auto docs)
composer require api-platform/core

# Serializer
composer require symfony/serializer

# Validation
composer require symfony/validator
```

### 2. Generate JWT Keys

```bash
# Generate JWT keypair
php bin/console lexik:jwt:generate-keypair

# Keys saved to:
# config/jwt/private.pem
# config/jwt/public.pem
```

### 3. Configure JWT Authentication

```yaml
# config/packages/lexik_jwt_authentication.yaml

lexik_jwt_authentication:
    secret_key: '%env(resolve:JWT_SECRET_KEY)%'
    public_key: '%env(resolve:JWT_PUBLIC_KEY)%'
    pass_phrase: '%env(JWT_PASSPHRASE)%'
    token_ttl: 3600  # 1 hour
    user_identity_field: email
```

### 4. Configure CORS

```yaml
# config/packages/nelmio_cors.yaml

nelmio_cors:
    defaults:
        origin_regex: true
        allow_origin: ['*']  # Restrict in production
        allow_methods: ['GET', 'POST', 'PUT', 'DELETE', 'OPTIONS']
        allow_headers: ['Content-Type', 'Authorization']
        expose_headers: ['Link']
        max_age: 3600
    paths:
        '^/api/':
            allow_origin: ['*']
            allow_headers: ['*']
            allow_methods: ['POST', 'PUT', 'GET', 'DELETE', 'OPTIONS']
            max_age: 3600
```

### 5. Security Configuration

```yaml
# config/packages/security.yaml

security:
    password_hashers:
        Symfony\Component\Security\Core\User\PasswordAuthenticatedUserInterface: 'auto'

    providers:
        app_user_provider:
            entity:
                class: App\Entity\User
                property: email

    firewalls:
        dev:
            pattern: ^/(_(profiler|wdt)|css|images|js)/
            security: false

        api_login:
            pattern: ^/api/v1/auth/token$
            stateless: true
            json_login:
                check_path: /api/v1/auth/token
                username_path: email
                password_path: password
                success_handler: lexik_jwt_authentication.handler.authentication_success
                failure_handler: lexik_jwt_authentication.handler.authentication_failure

        api:
            pattern: ^/api/v1
            stateless: true
            jwt: ~

    access_control:
        - { path: ^/api/v1/auth/token$, roles: PUBLIC_ACCESS }
        - { path: ^/api/v1, roles: IS_AUTHENTICATED_FULLY }
```

### 6. API Routes

```yaml
# config/routes/api.yaml

api:
    resource: '../src/Controller/Api/'
    type: attribute
    prefix: /api/v1
```

---

## 👥 CRM Contact API Controller

```php
<?php
// src/Controller/Api/CrmContactApiController.php

namespace App\Controller\Api;

use App\Entity\CRM\Contact;
use App\Repository\CRM\ContactRepository;
use Doctrine\ORM\EntityManagerInterface;
use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\Routing\Annotation\Route;
use Symfony\Component\Serializer\SerializerInterface;
use Symfony\Component\Validator\Validator\ValidatorInterface;

#[Route('/crm/contacts')]
class CrmContactApiController extends AbstractController
{
    public function __construct(
        private ContactRepository $contactRepository,
        private EntityManagerInterface $em,
        private SerializerInterface $serializer,
        private ValidatorInterface $validator,
    ) {}

    /**
     * List contacts with pagination and filters
     */
    #[Route('', name: 'api_crm_contacts_list', methods: ['GET'])]
    public function list(Request $request): JsonResponse
    {
        $user = $this->getUser();
        $tenantId = $user->getCurrentTenant()->getId();

        $page = $request->query->getInt('page', 1);
        $limit = min($request->query->getInt('limit', 50), 100);
        $search = $request->query->get('search');
        $tags = $request->query->get('tags') ? explode(',', $request->query->get('tags')) : null;
        $updatedSince = $request->query->get('updated_since');

        $qb = $this->contactRepository->createQueryBuilder('c')
            ->where('c.tenantId = :tenantId')
            ->setParameter('tenantId', $tenantId)
            ->orderBy('c.lastName', 'ASC');

        // Search filter
        if ($search) {
            $qb->andWhere(
                $qb->expr()->orX(
                    $qb->expr()->like('c.firstName', ':search'),
                    $qb->expr()->like('c.lastName', ':search'),
                    $qb->expr()->like('c.email', ':search'),
                    $qb->expr()->like('c.phone', ':search')
                )
            )->setParameter('search', '%' . $search . '%');
        }

        // Tags filter
        if ($tags) {
            $qb->andWhere('c.tags @> :tags')
                ->setParameter('tags', json_encode($tags));
        }

        // Incremental sync
        if ($updatedSince) {
            $qb->andWhere('c.updatedAt > :updatedSince')
                ->setParameter('updatedSince', new \DateTime($updatedSince));
        }

        // Pagination
        $total = count($qb->getQuery()->getResult());
        $contacts = $qb->setFirstResult(($page - 1) * $limit)
            ->setMaxResults($limit)
            ->getQuery()
            ->getResult();

        $data = $this->serializer->normalize($contacts, null, [
            'groups' => ['contact:list']
        ]);

        return $this->json([
            'contacts' => $data,
            'pagination' => [
                'page' => $page,
                'limit' => $limit,
                'total' => $total,
                'has_more' => ($page * $limit) < $total,
            ],
        ]);
    }

    /**
     * Get single contact by ID
     */
    #[Route('/{id}', name: 'api_crm_contacts_show', methods: ['GET'])]
    public function show(int $id): JsonResponse
    {
        $contact = $this->contactRepository->find($id);

        if (!$contact) {
            return $this->json(['error' => 'Contact not found'], Response::HTTP_NOT_FOUND);
        }

        // Check tenant access
        if ($contact->getTenantId() !== $this->getUser()->getCurrentTenant()->getId()) {
            return $this->json(['error' => 'Access denied'], Response::HTTP_FORBIDDEN);
        }

        $data = $this->serializer->normalize($contact, null, [
            'groups' => ['contact:detail']
        ]);

        return $this->json($data);
    }

    /**
     * Create new contact
     */
    #[Route('', name: 'api_crm_contacts_create', methods: ['POST'])]
    public function create(Request $request): JsonResponse
    {
        $data = json_decode($request->getContent(), true);

        $contact = new Contact();
        $contact->setTenantId($this->getUser()->getCurrentTenant()->getId());
        $contact->setFirstName($data['first_name'] ?? '');
        $contact->setLastName($data['last_name'] ?? '');
        $contact->setEmail($data['email'] ?? null);
        $contact->setPhone($data['phone'] ?? null);
        $contact->setCompany($data['company'] ?? null);
        $contact->setTitle($data['title'] ?? null);
        $contact->setTags($data['tags'] ?? []);
        $contact->setCustomFields($data['custom_fields'] ?? []);
        $contact->setNotes($data['notes'] ?? null);
        $contact->setLeadScore($data['lead_score'] ?? 0);

        // Validate
        $errors = $this->validator->validate($contact);
        if (count($errors) > 0) {
            return $this->json([
                'error' => 'Validation failed',
                'details' => (string) $errors
            ], Response::HTTP_UNPROCESSABLE_ENTITY);
        }

        $this->em->persist($contact);
        $this->em->flush();

        $responseData = $this->serializer->normalize($contact, null, [
            'groups' => ['contact:detail']
        ]);

        return $this->json($responseData, Response::HTTP_CREATED);
    }

    /**
     * Update contact
     */
    #[Route('/{id}', name: 'api_crm_contacts_update', methods: ['PUT'])]
    public function update(int $id, Request $request): JsonResponse
    {
        $contact = $this->contactRepository->find($id);

        if (!$contact) {
            return $this->json(['error' => 'Contact not found'], Response::HTTP_NOT_FOUND);
        }

        // Check tenant access
        if ($contact->getTenantId() !== $this->getUser()->getCurrentTenant()->getId()) {
            return $this->json(['error' => 'Access denied'], Response::HTTP_FORBIDDEN);
        }

        $data = json_decode($request->getContent(), true);

        // Update fields
        if (isset($data['first_name'])) $contact->setFirstName($data['first_name']);
        if (isset($data['last_name'])) $contact->setLastName($data['last_name']);
        if (isset($data['email'])) $contact->setEmail($data['email']);
        if (isset($data['phone'])) $contact->setPhone($data['phone']);
        if (isset($data['company'])) $contact->setCompany($data['company']);
        if (isset($data['title'])) $contact->setTitle($data['title']);
        if (isset($data['tags'])) $contact->setTags($data['tags']);
        if (isset($data['custom_fields'])) $contact->setCustomFields($data['custom_fields']);
        if (isset($data['notes'])) $contact->setNotes($data['notes']);
        if (isset($data['lead_score'])) $contact->setLeadScore($data['lead_score']);

        $this->em->flush();

        $responseData = $this->serializer->normalize($contact, null, [
            'groups' => ['contact:detail']
        ]);

        return $this->json($responseData);
    }

    /**
     * Delete contact
     */
    #[Route('/{id}', name: 'api_crm_contacts_delete', methods: ['DELETE'])]
    public function delete(int $id): JsonResponse
    {
        $contact = $this->contactRepository->find($id);

        if (!$contact) {
            return $this->json(['error' => 'Contact not found'], Response::HTTP_NOT_FOUND);
        }

        // Check tenant access
        if ($contact->getTenantId() !== $this->getUser()->getCurrentTenant()->getId()) {
            return $this->json(['error' => 'Access denied'], Response::HTTP_FORBIDDEN);
        }

        $this->em->remove($contact);
        $this->em->flush();

        return $this->json([
            'status' => 'deleted',
            'deleted_at' => (new \DateTime())->format('c'),
        ]);
    }

    /**
     * Bulk operations
     */
    #[Route('/bulk', name: 'api_crm_contacts_bulk', methods: ['POST'])]
    public function bulk(Request $request): JsonResponse
    {
        $data = json_decode($request->getContent(), true);
        $action = $data['action'] ?? null;
        $contactIds = $data['contact_ids'] ?? [];
        $payload = $data['data'] ?? [];

        $results = [];
        $processed = 0;
        $failed = 0;

        foreach ($contactIds as $id) {
            try {
                $contact = $this->contactRepository->find($id);

                if (!$contact || $contact->getTenantId() !== $this->getUser()->getCurrentTenant()->getId()) {
                    $results[] = ['id' => $id, 'status' => 'failed', 'error' => 'Not found'];
                    $failed++;
                    continue;
                }

                switch ($action) {
                    case 'update_tags':
                        $contact->setTags($payload['tags']);
                        break;
                    case 'delete':
                        $this->em->remove($contact);
                        break;
                    default:
                        $results[] = ['id' => $id, 'status' => 'failed', 'error' => 'Unknown action'];
                        $failed++;
                        continue 2;
                }

                $this->em->flush();
                $results[] = ['id' => $id, 'status' => 'success'];
                $processed++;

            } catch (\Exception $e) {
                $results[] = ['id' => $id, 'status' => 'failed', 'error' => $e->getMessage()];
                $failed++;
            }
        }

        return $this->json([
            'processed' => $processed,
            'failed' => $failed,
            'results' => $results,
        ]);
    }

    /**
     * Search contacts
     */
    #[Route('/search', name: 'api_crm_contacts_search', methods: ['GET'])]
    public function search(Request $request): JsonResponse
    {
        $query = $request->query->get('q');
        $tenantId = $this->getUser()->getCurrentTenant()->getId();

        if (!$query || strlen($query) < 2) {
            return $this->json(['error' => 'Query too short'], Response::HTTP_BAD_REQUEST);
        }

        $contacts = $this->contactRepository->searchFullText($query, $tenantId);

        $data = $this->serializer->normalize($contacts, null, [
            'groups' => ['contact:list']
        ]);

        return $this->json([
            'results' => $data,
            'total' => count($contacts),
        ]);
    }
}
```

---

## 📧 Email API Controller

```php
<?php
// src/Controller/Api/EmailMessageApiController.php

namespace App\Controller\Api;

use App\Entity\Email\EmailMessage;
use App\Repository\Email\EmailMessageRepository;
use App\Service\Email\EmailSyncService;
use Doctrine\ORM\EntityManagerInterface;
use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\Routing\Annotation\Route;
use Symfony\Component\Serializer\SerializerInterface;

#[Route('/email')]
class EmailMessageApiController extends AbstractController
{
    public function __construct(
        private EmailMessageRepository $messageRepository,
        private EmailSyncService $syncService,
        private EntityManagerInterface $em,
        private SerializerInterface $serializer,
    ) {}

    /**
     * List messages in folder
     */
    #[Route('/folders/{folderId}/messages', name: 'api_email_messages_list', methods: ['GET'])]
    public function listMessages(int $folderId, Request $request): JsonResponse
    {
        $page = $request->query->getInt('page', 1);
        $limit = min($request->query->getInt('limit', 50), 100);
        $sinceUid = $request->query->get('since_uid');

        $qb = $this->messageRepository->createQueryBuilder('m')
            ->where('m.folder = :folderId')
            ->setParameter('folderId', $folderId)
            ->orderBy('m.receivedDate', 'DESC');

        // Incremental sync
        if ($sinceUid) {
            $qb->andWhere('m.uid > :sinceUid')
                ->setParameter('sinceUid', $sinceUid);
        }

        $total = count($qb->getQuery()->getResult());
        $messages = $qb->setFirstResult(($page - 1) * $limit)
            ->setMaxResults($limit)
            ->getQuery()
            ->getResult();

        $data = $this->serializer->normalize($messages, null, [
            'groups' => ['message:list']
        ]);

        return $this->json([
            'messages' => $data,
            'pagination' => [
                'page' => $page,
                'limit' => $limit,
                'total' => $total,
                'has_more' => ($page * $limit) < $total,
            ],
        ]);
    }

    /**
     * Get full message
     */
    #[Route('/messages/{id}', name: 'api_email_messages_show', methods: ['GET'])]
    public function showMessage(int $id): JsonResponse
    {
        $message = $this->messageRepository->find($id);

        if (!$message) {
            return $this->json(['error' => 'Message not found'], Response::HTTP_NOT_FOUND);
        }

        $data = $this->serializer->normalize($message, null, [
            'groups' => ['message:detail']
        ]);

        return $this->json($data);
    }

    /**
     * Send email
     */
    #[Route('/messages', name: 'api_email_messages_send', methods: ['POST'])]
    public function sendMessage(Request $request): JsonResponse
    {
        $data = json_decode($request->getContent(), true);

        $accountId = $data['account_id'];
        $to = $data['to'] ?? [];
        $cc = $data['cc'] ?? [];
        $bcc = $data['bcc'] ?? [];
        $subject = $data['subject'] ?? '';
        $bodyHtml = $data['body_html'] ?? '';
        $bodyPlain = $data['body_plain'] ?? strip_tags($bodyHtml);
        $attachments = $data['attachments'] ?? [];

        try {
            $messageId = $this->syncService->sendEmail(
                $accountId,
                $to,
                $subject,
                $bodyHtml,
                $bodyPlain,
                $cc,
                $bcc,
                $attachments
            );

            return $this->json([
                'status' => 'sent',
                'message_id' => $messageId,
                'sent_at' => (new \DateTime())->format('c'),
            ], Response::HTTP_CREATED);

        } catch (\Exception $e) {
            return $this->json([
                'error' => 'Failed to send email',
                'message' => $e->getMessage(),
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Update message flags
     */
    #[Route('/messages/{id}/flags', name: 'api_email_messages_flags', methods: ['PUT'])]
    public function updateFlags(int $id, Request $request): JsonResponse
    {
        $message = $this->messageRepository->find($id);

        if (!$message) {
            return $this->json(['error' => 'Message not found'], Response::HTTP_NOT_FOUND);
        }

        $data = json_decode($request->getContent(), true);

        if (isset($data['is_read'])) {
            $message->setIsRead($data['is_read']);
        }

        if (isset($data['is_flagged'])) {
            $message->setIsFlagged($data['is_flagged']);
        }

        $this->em->flush();

        // Update on mail server
        $this->syncService->updateMessageFlags($message);

        return $this->json([
            'id' => $message->getId(),
            'is_read' => $message->isRead(),
            'is_flagged' => $message->isFlagged(),
            'updated_at' => $message->getUpdatedAt()->format('c'),
        ]);
    }

    /**
     * Move message to folder
     */
    #[Route('/messages/{id}/move', name: 'api_email_messages_move', methods: ['POST'])]
    public function moveMessage(int $id, Request $request): JsonResponse
    {
        $message = $this->messageRepository->find($id);

        if (!$message) {
            return $this->json(['error' => 'Message not found'], Response::HTTP_NOT_FOUND);
        }

        $data = json_decode($request->getContent(), true);
        $targetFolderId = $data['target_folder_id'];

        $targetFolder = $this->em->getRepository('App:Email\EmailFolder')->find($targetFolderId);

        if (!$targetFolder) {
            return $this->json(['error' => 'Target folder not found'], Response::HTTP_NOT_FOUND);
        }

        $message->setFolder($targetFolder);
        $this->em->flush();

        // Move on mail server
        $this->syncService->moveMessage($message, $targetFolder);

        return $this->json([
            'id' => $message->getId(),
            'folder_id' => $targetFolder->getId(),
            'moved_at' => (new \DateTime())->format('c'),
        ]);
    }

    /**
     * Delete message
     */
    #[Route('/messages/{id}', name: 'api_email_messages_delete', methods: ['DELETE'])]
    public function deleteMessage(int $id): JsonResponse
    {
        $message = $this->messageRepository->find($id);

        if (!$message) {
            return $this->json(['error' => 'Message not found'], Response::HTTP_NOT_FOUND);
        }

        $message->setIsDeleted(true);
        $this->em->flush();

        // Delete on mail server
        $this->syncService->deleteMessage($message);

        return $this->json([
            'status' => 'deleted',
            'deleted_at' => (new \DateTime())->format('c'),
        ]);
    }
}
```

---

## 📅 Calendar API Controller

```php
<?php
// src/Controller/Api/CalendarApiController.php

namespace App\Controller\Api;

use App\Entity\Calendar\CalendarEvent;
use App\Repository\Calendar\CalendarEventRepository;
use Doctrine\ORM\EntityManagerInterface;
use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\Routing\Annotation\Route;
use Symfony\Component\Serializer\SerializerInterface;

#[Route('/calendar')]
class CalendarApiController extends AbstractController
{
    public function __construct(
        private CalendarEventRepository $eventRepository,
        private EntityManagerInterface $em,
        private SerializerInterface $serializer,
    ) {}

    /**
     * List events within date range
     */
    #[Route('/events', name: 'api_calendar_events_list', methods: ['GET'])]
    public function listEvents(Request $request): JsonResponse
    {
        $start = $request->query->get('start');
        $end = $request->query->get('end');
        $tenantId = $this->getUser()->getCurrentTenant()->getId();

        $qb = $this->eventRepository->createQueryBuilder('e')
            ->where('e.tenantId = :tenantId')
            ->setParameter('tenantId', $tenantId);

        if ($start) {
            $qb->andWhere('e.startTime >= :start')
                ->setParameter('start', new \DateTime($start));
        }

        if ($end) {
            $qb->andWhere('e.endTime <= :end')
                ->setParameter('end', new \DateTime($end));
        }

        $events = $qb->orderBy('e.startTime', 'ASC')
            ->getQuery()
            ->getResult();

        $data = $this->serializer->normalize($events, null, [
            'groups' => ['event:list']
        ]);

        return $this->json(['events' => $data]);
    }

    /**
     * Create event
     */
    #[Route('/events', name: 'api_calendar_events_create', methods: ['POST'])]
    public function createEvent(Request $request): JsonResponse
    {
        $data = json_decode($request->getContent(), true);

        $event = new CalendarEvent();
        $event->setTenantId($this->getUser()->getCurrentTenant()->getId());
        $event->setTitle($data['title']);
        $event->setDescription($data['description'] ?? null);
        $event->setLocation($data['location'] ?? null);
        $event->setStartTime(new \DateTime($data['start_time']));
        $event->setEndTime(new \DateTime($data['end_time']));
        $event->setAllDay($data['all_day'] ?? false);
        $event->setAttendees($data['attendees'] ?? []);

        $this->em->persist($event);
        $this->em->flush();

        $responseData = $this->serializer->normalize($event, null, [
            'groups' => ['event:detail']
        ]);

        return $this->json($responseData, Response::HTTP_CREATED);
    }

    /**
     * Update event
     */
    #[Route('/events/{id}', name: 'api_calendar_events_update', methods: ['PUT'])]
    public function updateEvent(int $id, Request $request): JsonResponse
    {
        $event = $this->eventRepository->find($id);

        if (!$event) {
            return $this->json(['error' => 'Event not found'], Response::HTTP_NOT_FOUND);
        }

        $data = json_decode($request->getContent(), true);

        if (isset($data['title'])) $event->setTitle($data['title']);
        if (isset($data['description'])) $event->setDescription($data['description']);
        if (isset($data['location'])) $event->setLocation($data['location']);
        if (isset($data['start_time'])) $event->setStartTime(new \DateTime($data['start_time']));
        if (isset($data['end_time'])) $event->setEndTime(new \DateTime($data['end_time']));
        if (isset($data['all_day'])) $event->setAllDay($data['all_day']);
        if (isset($data['attendees'])) $event->setAttendees($data['attendees']);

        $this->em->flush();

        $responseData = $this->serializer->normalize($event, null, [
            'groups' => ['event:detail']
        ]);

        return $this->json($responseData);
    }

    /**
     * Delete event
     */
    #[Route('/events/{id}', name: 'api_calendar_events_delete', methods: ['DELETE'])]
    public function deleteEvent(int $id): JsonResponse
    {
        $event = $this->eventRepository->find($id);

        if (!$event) {
            return $this->json(['error' => 'Event not found'], Response::HTTP_NOT_FOUND);
        }

        $this->em->remove($event);
        $this->em->flush();

        return $this->json([
            'status' => 'deleted',
            'deleted_at' => (new \DateTime())->format('c'),
        ]);
    }
}
```

---

## 🔐 Authentication & Security

### Rate Limiting

```php
<?php
// src/EventListener/RateLimitListener.php

namespace App\EventListener;

use Symfony\Component\HttpKernel\Event\RequestEvent;
use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Contracts\Cache\CacheInterface;

class RateLimitListener
{
    private const MAX_REQUESTS = 100;
    private const TIME_WINDOW = 3600; // 1 hour

    public function __construct(
        private CacheInterface $cache,
    ) {}

    public function onKernelRequest(RequestEvent $event): void
    {
        $request = $event->getRequest();

        if (!str_starts_with($request->getPathInfo(), '/api/v1')) {
            return;
        }

        $userId = $request->attributes->get('user_id');
        if (!$userId) {
            return;
        }

        $key = "rate_limit_{$userId}";
        $item = $this->cache->getItem($key);

        if (!$item->isHit()) {
            $item->set(1);
            $item->expiresAfter(self::TIME_WINDOW);
            $this->cache->save($item);
            return;
        }

        $count = $item->get();

        if ($count >= self::MAX_REQUESTS) {
            $event->setResponse(new JsonResponse([
                'error' => 'Rate limit exceeded',
                'retry_after' => self::TIME_WINDOW,
            ], Response::HTTP_TOO_MANY_REQUESTS));
            return;
        }

        $item->set($count + 1);
        $this->cache->save($item);
    }
}
```

---

## 🧪 Testing

### API Test Example

```php
<?php
// tests/Api/CrmContactApiTest.php

namespace App\Tests\Api;

use Symfony\Bundle\FrameworkBundle\Test\WebTestCase;

class CrmContactApiTest extends WebTestCase
{
    private $client;
    private $token;

    protected function setUp(): void
    {
        $this->client = static::createClient();

        // Get JWT token
        $this->client->request('POST', '/api/v1/auth/token', [], [], [
            'CONTENT_TYPE' => 'application/json',
        ], json_encode([
            'email' => 'test@example.com',
            'password' => 'password',
        ]));

        $response = json_decode($this->client->getResponse()->getContent(), true);
        $this->token = $response['token'];
    }

    public function testListContacts(): void
    {
        $this->client->request('GET', '/api/v1/crm/contacts', [], [], [
            'HTTP_AUTHORIZATION' => 'Bearer ' . $this->token,
        ]);

        $this->assertResponseIsSuccessful();
        $this->assertResponseHeaderSame('content-type', 'application/json');

        $data = json_decode($this->client->getResponse()->getContent(), true);
        $this->assertArrayHasKey('contacts', $data);
        $this->assertArrayHasKey('pagination', $data);
    }

    public function testCreateContact(): void
    {
        $this->client->request('POST', '/api/v1/crm/contacts', [], [], [
            'HTTP_AUTHORIZATION' => 'Bearer ' . $this->token,
            'CONTENT_TYPE' => 'application/json',
        ], json_encode([
            'first_name' => 'John',
            'last_name' => 'Doe',
            'email' => 'john@example.com',
            'tags' => ['lead'],
        ]));

        $this->assertResponseStatusCodeSame(201);

        $data = json_decode($this->client->getResponse()->getContent(), true);
        $this->assertEquals('John', $data['first_name']);
        $this->assertEquals('Doe', $data['last_name']);
    }
}
```

---

## ✅ Implementation Checklist

**Setup:**
- [ ] Install JWT bundle
- [ ] Generate JWT keys
- [ ] Configure CORS
- [ ] Set up security.yaml

**CRM API:**
- [ ] Create CrmContactApiController
- [ ] Implement list/search endpoints
- [ ] Implement CRUD operations
- [ ] Add bulk operations
- [ ] Write tests

**Email API:**
- [ ] Create EmailMessageApiController
- [ ] Implement message listing
- [ ] Implement send/reply
- [ ] Add flag operations
- [ ] Write tests

**Calendar API:**
- [ ] Create CalendarApiController
- [ ] Implement event CRUD
- [ ] Add date range filtering
- [ ] Write tests

**Production:**
- [ ] Add rate limiting
- [ ] Add logging
- [ ] Add error handling
- [ ] Deploy to production

---

**Next**: Frontend implementation with Tauri + Vue 3
