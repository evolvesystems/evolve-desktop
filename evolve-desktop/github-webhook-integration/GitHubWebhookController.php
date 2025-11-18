<?php

namespace App\Controller\Webhook;

use App\Entity\AppRelease;
use App\Repository\AppReleaseRepository;
use Doctrine\ORM\EntityManagerInterface;
use Psr\Log\LoggerInterface;
use Symfony\Bundle\FrameworkBundle\Controller\AbstractController;
use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\Routing\Attribute\Route;

/**
 * GitHub Webhook Controller for Desktop App Releases
 *
 * This controller handles GitHub webhook events for:
 * 1. Release published events - Automatically creates/updates app releases in admin panel
 * 2. Workflow run events - Monitors build status in real-time
 *
 * Setup Instructions:
 * 1. Copy this file to: /home/john/sources/eiq-manager/src/Controller/Webhook/GitHubWebhookController.php
 * 2. Add GITHUB_WEBHOOK_SECRET to your .env file
 * 3. Configure GitHub webhook at: https://github.com/evolvesystems/evolve-desktop/settings/hooks
 *    - Payload URL: http://localhost:8547/webhook/github/release
 *    - Content type: application/json
 *    - Secret: (use the same value as GITHUB_WEBHOOK_SECRET)
 *    - Events: Release events, Workflow run events
 */
#[Route('/webhook')]
class GitHubWebhookController extends AbstractController
{
    /**
     * Handle GitHub Release Published Events
     *
     * When a release is published on GitHub, this endpoint:
     * - Extracts version, release notes, and assets
     * - Maps platform-specific installers (Windows .msi, Linux .deb/.AppImage, macOS .dmg)
     * - Creates or updates the AppRelease entity
     * - Automatically marks latest stable releases
     */
    #[Route('/github/release', name: 'webhook_github_release', methods: ['POST'])]
    public function handleRelease(
        Request $request,
        EntityManagerInterface $em,
        AppReleaseRepository $repo,
        LoggerInterface $logger
    ): JsonResponse {
        // Verify GitHub signature for security
        $signature = $request->headers->get('X-Hub-Signature-256');
        $payload = $request->getContent();

        // Get webhook secret from env
        $secret = $_ENV['GITHUB_WEBHOOK_SECRET'] ?? null;

        if ($secret) {
            $expectedSignature = 'sha256=' . hash_hmac('sha256', $payload, $secret);

            if (!hash_equals($expectedSignature, $signature ?? '')) {
                $logger->warning('GitHub webhook signature mismatch');
                return new JsonResponse(['error' => 'Invalid signature'], Response::HTTP_UNAUTHORIZED);
            }
        }

        $data = json_decode($payload, true);

        // Only handle release published events
        if (($data['action'] ?? null) !== 'published') {
            return new JsonResponse(['message' => 'Event ignored, not a published release'], Response::HTTP_OK);
        }

        try {
            $release = $data['release'] ?? [];
            $version = ltrim($release['tag_name'] ?? '', 'v'); // Remove 'v' prefix if present

            $logger->info("Processing GitHub release webhook for version: {$version}");

            // Check if release already exists
            $appRelease = $repo->findOneBy(['version' => $version]);

            if (!$appRelease) {
                $appRelease = new AppRelease();
                $appRelease->setVersion($version);
                $logger->info("Creating new app release for version: {$version}");
            } else {
                $logger->info("Updating existing app release for version: {$version}");
            }

            // Set release data
            $appRelease->setReleaseNotes($release['body'] ?? '');
            $appRelease->setReleaseDate(new \DateTime($release['published_at'] ?? 'now'));
            $appRelease->setIsStable(!($release['prerelease'] ?? false));

            // Parse assets to find platform-specific installers
            $assets = $release['assets'] ?? [];

            foreach ($assets as $asset) {
                $name = strtolower($asset['name'] ?? '');
                $downloadUrl = $asset['browser_download_url'] ?? '';

                if (empty($downloadUrl)) {
                    continue;
                }

                // Windows installers (.msi or setup.exe)
                if (str_contains($name, '.msi') || (str_contains($name, '.exe') && str_contains($name, 'setup'))) {
                    $appRelease->setWindowsUrl($downloadUrl);
                    $logger->info("Found Windows installer: {$name}");
                }

                // Linux Debian packages
                if (str_contains($name, '.deb')) {
                    $appRelease->setLinuxDebUrl($downloadUrl);
                    $logger->info("Found Linux DEB package: {$name}");
                }

                // Linux AppImage
                if (str_contains($name, '.appimage')) {
                    $appRelease->setLinuxAppimageUrl($downloadUrl);
                    $logger->info("Found Linux AppImage: {$name}");
                }

                // macOS installers
                if (str_contains($name, '.dmg') || str_contains($name, '.pkg')) {
                    $appRelease->setMacosUrl($downloadUrl);
                    $logger->info("Found macOS installer: {$name}");
                }
            }

            // If this is a stable release (not prerelease), mark it as latest
            if (($release['draft'] ?? false) === false && ($release['prerelease'] ?? false) === false) {
                // Unmark all other releases as latest
                $allReleases = $repo->findAll();
                foreach ($allReleases as $r) {
                    if ($r->getId() !== $appRelease->getId()) {
                        $r->setIsLatest(false);
                    }
                }

                $appRelease->setIsLatest(true);
                $logger->info("Marked version {$version} as latest release");
            }

            $em->persist($appRelease);
            $em->flush();

            $logger->info("Successfully processed GitHub release for version: {$version}");

            return new JsonResponse([
                'success' => true,
                'message' => "Release {$version} processed successfully",
                'version' => $version,
                'platforms' => [
                    'windows' => !empty($appRelease->getWindowsUrl()),
                    'linux_deb' => !empty($appRelease->getLinuxDebUrl()),
                    'linux_appimage' => !empty($appRelease->getLinuxAppimageUrl()),
                    'macos' => !empty($appRelease->getMacosUrl()),
                ]
            ], Response::HTTP_OK);

        } catch (\Exception $e) {
            $logger->error('Error processing GitHub release webhook: ' . $e->getMessage(), [
                'exception' => $e,
                'payload' => $data ?? []
            ]);

            return new JsonResponse([
                'error' => 'Failed to process release',
                'message' => $e->getMessage()
            ], Response::HTTP_INTERNAL_SERVER_ERROR);
        }
    }

    /**
     * Handle GitHub Workflow Run Events
     *
     * Monitors build status in real-time for logging/debugging
     */
    #[Route('/github/workflow', name: 'webhook_github_workflow', methods: ['POST'])]
    public function handleWorkflow(
        Request $request,
        LoggerInterface $logger
    ): JsonResponse {
        // Verify GitHub signature
        $signature = $request->headers->get('X-Hub-Signature-256');
        $payload = $request->getContent();

        $secret = $_ENV['GITHUB_WEBHOOK_SECRET'] ?? null;

        if ($secret) {
            $expectedSignature = 'sha256=' . hash_hmac('sha256', $payload, $secret);

            if (!hash_equals($expectedSignature, $signature ?? '')) {
                $logger->warning('GitHub webhook signature mismatch');
                return new JsonResponse(['error' => 'Invalid signature'], Response::HTTP_UNAUTHORIZED);
            }
        }

        $data = json_decode($payload, true);

        // Log workflow status for monitoring
        $logger->info('GitHub Actions workflow event received', [
            'action' => $data['action'] ?? 'unknown',
            'workflow' => $data['workflow']['name'] ?? 'unknown',
            'status' => $data['workflow_run']['status'] ?? 'unknown',
            'conclusion' => $data['workflow_run']['conclusion'] ?? null,
        ]);

        return new JsonResponse(['success' => true], Response::HTTP_OK);
    }
}
